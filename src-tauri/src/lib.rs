mod command;
mod error;
mod structs;

use crate::error::Exception;
use command::*;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
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
        .invoke_handler(tauri::generate_handler![greet, list_transactions_by_day])
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
    let mut is_not_exists = false;
    if !url_path.exists() {
        create_dir_all("db")?;
    }
    if !url_path.exists() {
        std::fs::File::create_new(url_path)?;
        is_not_exists = true;
    }
    let url = format!("sqlite://{}", db_name);
    let pool = SqlitePoolOptions::new()
        .max_connections(100)
        .connect(&url)
        .await?;
    if is_not_exists {
        sqlx::query_file_unchecked!("db/table.sql")
            .execute(&pool)
            .await?;
    }
    Ok(pool)
}
