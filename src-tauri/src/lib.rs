mod command;
mod error;
mod structs;

use crate::error::Exception;
use command::*;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Row, Sqlite};
use std::fs::create_dir_all;
use std::path::Path;
use tauri::Manager;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<(), Exception> {
    dotenv::dotenv().ok();
    init_log()?;
    let pool = get_connection().await?;
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .setup(|app| {
            let _app_handle = app.app_handle();

            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            list_transactions_by_day,
            list_transactions_by_month,
            list_ledgers,
            list_categories,
            list_categories_by_type,
            list_tags,
            list_assets,
            add_transaction
        ])
        .manage(pool)
        .run(tauri::generate_context!())
        .map_err(|e| Exception::RuntimeException(format!("启动应用失败: {}", e)))?;
    Ok(())
}

/// 初始化日志
fn init_log() -> Result<(), Exception> {
    let now_time_str = format!("{}", chrono::Local::now().format("%Y_%m_%d"));
    let log_path = format!("logs/{}.log", now_time_str);
    let log_path = Path::new(&log_path);
    if let Some(parent_path) = log_path.parent() {
        if !parent_path.exists() {
            std::fs::create_dir(parent_path)?;
        }
    }

    let file = std::fs::File::options()
        .append(true)
        .create(true)
        .open(log_path)?;

    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_timer(tracing_subscriber::fmt::time::time())
        .with_level(true)
        .with_target(true)
        .with_line_number(true)
        .with_ansi(true)
        .pretty();

    let file_layer = fmt::layer()
        .with_writer(file)
        .with_file(true)
        .with_timer(tracing_subscriber::fmt::time::time())
        .with_target(true)
        .with_level(true)
        .with_line_number(true)
        .with_ansi(true)
        .pretty();

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .with(EnvFilter::from_default_env())
        .init();
    Ok(())
}

/// 获取数据库连接池
async fn get_connection() -> Result<Pool<Sqlite>, Exception> {
    let db_name = "db/test.db";
    let url_path = Path::new(db_name);
    if !url_path.exists() {
        create_dir_all("db")?;
    }
    let url = format!("sqlite://{}", db_name);
    let pool = SqlitePoolOptions::new()
        .max_connections(100)
        .connect(&url)
        .await?;
    init_database(&pool).await?;
    Ok(pool)
}

async fn init_database(pool: &Pool<Sqlite>) -> Result<(), Exception> {
    create_tables(pool).await?;
    patch_legacy_schema(pool).await?;
    create_indexes(pool).await?;
    seed_default_data(pool).await?;
    Ok(())
}

async fn create_tables(pool: &Pool<Sqlite>) -> Result<(), Exception> {
    let statements = [
        r#"
        CREATE TABLE IF NOT EXISTS ledger (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            ledger_name     TEXT NOT NULL,
            ledger_type     INTEGER NOT NULL DEFAULT 0,
            ledger_image    TEXT NOT NULL DEFAULT '',
            create_time     TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            update_time     TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS category (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            name            TEXT NOT NULL,
            parent_id       INTEGER REFERENCES category(id) ON DELETE SET NULL,
            image           TEXT NOT NULL DEFAULT '',
            color           TEXT NOT NULL DEFAULT '#8E8E93',
            ledger_type     INTEGER NOT NULL DEFAULT 0,
            sort_order      INTEGER NOT NULL DEFAULT 0,
            created_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS tags (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            name            TEXT NOT NULL UNIQUE,
            color           TEXT NOT NULL DEFAULT '#007AFF',
            created_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            ledger_id           INTEGER NOT NULL REFERENCES ledger(id) ON DELETE CASCADE,
            category_id         INTEGER NOT NULL REFERENCES category(id),
            amount              DECIMAL(12, 2) NOT NULL,
            cost_type           INTEGER NOT NULL DEFAULT 0,
            comment             TEXT,
            transaction_time    TEXT NOT NULL,
            created_at          TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at          TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS transaction_tags (
            transaction_id  INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
            tag_id          INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
            PRIMARY KEY (transaction_id, tag_id)
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS assets (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            name            TEXT NOT NULL,
            asset_type      INTEGER NOT NULL DEFAULT 0,
            balance         DECIMAL(12, 2) NOT NULL DEFAULT 0.00,
            color           TEXT NOT NULL DEFAULT '#34C759',
            icon            TEXT NOT NULL DEFAULT 'cash-outline.svg',
            ledger_id       INTEGER REFERENCES ledger(id) ON DELETE SET NULL,
            created_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        )
        "#,
    ];

    for statement in statements {
        sqlx::query(statement).execute(pool).await?;
    }

    Ok(())
}

async fn patch_legacy_schema(pool: &Pool<Sqlite>) -> Result<(), Exception> {
    add_column_if_missing(pool, "ledger", "update_time", "TEXT").await?;
    add_column_if_missing(pool, "category", "color", "TEXT").await?;
    add_column_if_missing(pool, "category", "ledger_type", "INTEGER").await?;
    add_column_if_missing(pool, "category", "sort_order", "INTEGER").await?;
    add_column_if_missing(pool, "category", "created_at", "TEXT").await?;
    add_column_if_missing(pool, "tags", "color", "TEXT").await?;
    add_column_if_missing(pool, "tags", "created_at", "TEXT").await?;
    add_column_if_missing(pool, "transactions", "cost_type", "INTEGER").await?;
    add_column_if_missing(pool, "transactions", "updated_at", "TEXT").await?;

    let normalize_statements = [
        "UPDATE ledger SET update_time = COALESCE(update_time, create_time, datetime('now', 'localtime'))",
        "UPDATE category SET color = COALESCE(color, '#8E8E93')",
        "UPDATE category SET ledger_type = COALESCE(ledger_type, 0)",
        "UPDATE category SET sort_order = COALESCE(sort_order, 0)",
        "UPDATE category SET created_at = COALESCE(created_at, datetime('now', 'localtime'))",
        "UPDATE tags SET color = COALESCE(color, '#007AFF')",
        "UPDATE tags SET created_at = COALESCE(created_at, datetime('now', 'localtime'))",
        "UPDATE transactions SET cost_type = COALESCE(cost_type, 0)",
        "UPDATE transactions SET updated_at = COALESCE(updated_at, created_at, datetime('now', 'localtime'))",
    ];

    for statement in normalize_statements {
        sqlx::query(statement).execute(pool).await?;
    }

    Ok(())
}

async fn create_indexes(pool: &Pool<Sqlite>) -> Result<(), Exception> {
    let statements = [
        "CREATE INDEX IF NOT EXISTS idx_ledger_type ON ledger(ledger_type)",
        "CREATE INDEX IF NOT EXISTS idx_category_parent ON category(parent_id)",
        "CREATE INDEX IF NOT EXISTS idx_category_ledger_type ON category(ledger_type)",
        "CREATE INDEX IF NOT EXISTS idx_transactions_ledger ON transactions(ledger_id)",
        "CREATE INDEX IF NOT EXISTS idx_transactions_category ON transactions(category_id)",
        "CREATE INDEX IF NOT EXISTS idx_transactions_time ON transactions(transaction_time)",
        "CREATE INDEX IF NOT EXISTS idx_transactions_cost_type ON transactions(cost_type)",
        "CREATE INDEX IF NOT EXISTS idx_transaction_tags_tag ON transaction_tags(tag_id)",
        "CREATE INDEX IF NOT EXISTS idx_assets_ledger ON assets(ledger_id)",
        "CREATE INDEX IF NOT EXISTS idx_assets_type ON assets(asset_type)",
    ];

    for statement in statements {
        sqlx::query(statement).execute(pool).await?;
    }

    Ok(())
}

async fn seed_default_data(pool: &Pool<Sqlite>) -> Result<(), Exception> {
    ensure_ledger(pool, "日常支出", 0, "wallet-outline.svg").await?;
    ensure_ledger(pool, "收入记录", 1, "cash-outline.svg").await?;
    ensure_ledger(pool, "资产账户", 2, "card-outline.svg").await?;

    let categories = [
        ("餐饮", "fast-food-outline.svg", "#FF9500", 0, 1),
        ("交通", "train-outline.svg", "#007AFF", 0, 2),
        ("购物", "bag-outline.svg", "#FF2D55", 0, 3),
        ("日用", "cart-outline.svg", "#8E8E93", 0, 4),
        ("娱乐", "film-outline.svg", "#AF52DE", 0, 5),
        ("住房", "home-outline.svg", "#5856D6", 0, 6),
        ("医疗", "medical-outline.svg", "#FF3B30", 0, 7),
        ("教育", "school-outline.svg", "#5AC8FA", 0, 8),
        ("礼物", "gift-outline.svg", "#FF2D55", 0, 9),
        ("旅行", "airplane-outline.svg", "#FFCC00", 0, 10),
        ("通讯", "phone-portrait-outline.svg", "#007AFF", 0, 11),
        ("护肤", "pricetag-outline.svg", "#FF2D55", 0, 12),
        ("工资", "cash-outline.svg", "#34C759", 1, 1),
        ("奖金", "ribbon-outline.svg", "#34C759", 1, 2),
        ("兼职", "today-outline.svg", "#34C759", 1, 3),
        ("理财", "cash-outline.svg", "#34C759", 1, 4),
        ("其他", "cash-outline.svg", "#8E8E93", 1, 5),
    ];

    for (name, image, color, ledger_type, sort_order) in categories {
        ensure_category(pool, name, image, color, ledger_type, sort_order).await?;
    }

    ensure_asset(pool, "现金", 0, 2000.00, "#34C759", "cash-outline.svg", Some(3)).await?;
    ensure_asset(pool, "银行卡", 1, 15000.00, "#007AFF", "card-outline.svg", Some(3)).await?;
    ensure_asset(pool, "信用卡", 2, -2000.00, "#FF3B30", "card-outline.svg", Some(3)).await?;

    Ok(())
}

async fn add_column_if_missing(
    pool: &Pool<Sqlite>,
    table: &str,
    column: &str,
    definition: &str,
) -> Result<(), Exception> {
    if !column_exists(pool, table, column).await? {
        let sql = format!("ALTER TABLE {table} ADD COLUMN {column} {definition}");
        sqlx::query(&sql).execute(pool).await?;
    }
    Ok(())
}

async fn column_exists(pool: &Pool<Sqlite>, table: &str, column: &str) -> Result<bool, Exception> {
    let sql = format!("PRAGMA table_info({table})");
    let rows = sqlx::query(&sql).fetch_all(pool).await?;
    Ok(rows.iter().any(|row| row.get::<String, _>("name") == column))
}

async fn ensure_ledger(
    pool: &Pool<Sqlite>,
    ledger_name: &str,
    ledger_type: i64,
    ledger_image: &str,
) -> Result<(), Exception> {
    let exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM ledger WHERE ledger_name = ? AND ledger_type = ?",
    )
    .bind(ledger_name)
    .bind(ledger_type)
    .fetch_one(pool)
    .await?;

    if exists == 0 {
        sqlx::query(
            "INSERT INTO ledger (ledger_name, ledger_type, ledger_image, create_time, update_time) VALUES (?, ?, ?, datetime('now', 'localtime'), datetime('now', 'localtime'))",
        )
        .bind(ledger_name)
        .bind(ledger_type)
        .bind(ledger_image)
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn ensure_category(
    pool: &Pool<Sqlite>,
    name: &str,
    image: &str,
    color: &str,
    ledger_type: i64,
    sort_order: i64,
) -> Result<(), Exception> {
    let exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM category WHERE name = ? AND ledger_type = ?",
    )
    .bind(name)
    .bind(ledger_type)
    .fetch_one(pool)
    .await?;

    if exists == 0 {
        sqlx::query(
            "INSERT INTO category (name, parent_id, image, color, ledger_type, sort_order, created_at) VALUES (?, NULL, ?, ?, ?, ?, datetime('now', 'localtime'))",
        )
        .bind(name)
        .bind(image)
        .bind(color)
        .bind(ledger_type)
        .bind(sort_order)
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn ensure_asset(
    pool: &Pool<Sqlite>,
    name: &str,
    asset_type: i64,
    balance: f64,
    color: &str,
    icon: &str,
    ledger_id: Option<i64>,
) -> Result<(), Exception> {
    let exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM assets WHERE name = ?")
        .bind(name)
        .fetch_one(pool)
        .await?;

    if exists == 0 {
        sqlx::query(
            "INSERT INTO assets (name, asset_type, balance, color, icon, ledger_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, datetime('now', 'localtime'), datetime('now', 'localtime'))",
        )
        .bind(name)
        .bind(asset_type)
        .bind(balance)
        .bind(color)
        .bind(icon)
        .bind(ledger_id)
        .execute(pool)
        .await?;
    }

    Ok(())
}
