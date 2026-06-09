use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{FromRow, Pool, Sqlite, Row};

/// 账本
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ledger {
    pub id: i64,
    pub ledger_name: String,
    pub ledger_type: i64,
    pub ledger_image: String,
    pub create_time: String,
    pub update_time: String,
}

/// 分类
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub image: String,
    pub color: String,
    pub ledger_type: i64,
    pub sort_order: i64,
    pub created_at: String,
}

/// 标签
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tags {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub created_at: String,
}

/// 交易-标签关联
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TransactionTags {
    pub transaction_id: i64,
    pub tag_id: i64,
}

/// 交易记录
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i64,
    pub ledger_id: i64,
    pub category_id: i64,
    pub amount: f64,
    pub cost_type: i64,
    pub comment: Option<String>,
    pub transaction_time: String,
    pub created_at: String,
    pub updated_at: String,
    // 非数据库字段
    #[sqlx(skip)]
    pub category_name: Option<String>,
    #[sqlx(skip)]
    pub image: Option<String>,
    #[sqlx(skip)]
    pub color: Option<String>,
}

/// 资产
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Asset {
    pub id: i64,
    pub name: String,
    pub asset_type: i64,
    pub balance: f64,
    pub color: String,
    pub icon: String,
    pub ledger_id: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

impl Ledger {
    pub async fn select_list(pool: &Pool<Sqlite>) -> Result<Vec<Ledger>, sqlx::Error> {
        sqlx::query_as::<_, Ledger>("SELECT * FROM ledger ORDER BY id")
            .fetch_all(pool)
            .await
    }

    pub async fn insert(
        pool: &Pool<Sqlite>,
        ledger_name: &str,
        ledger_type: i64,
        ledger_image: &str,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "INSERT INTO ledger (ledger_name, ledger_type, ledger_image) VALUES (?, ?, ?)",
        )
        .bind(ledger_name)
        .bind(ledger_type)
        .bind(ledger_image)
        .execute(pool)
        .await
    }
}

impl Category {
    pub async fn select_list(pool: &Pool<Sqlite>) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(
            "SELECT id, name, parent_id, image, color, ledger_type, sort_order, created_at FROM category ORDER BY sort_order, id"
        )
        .fetch_all(pool)
        .await
    }

    pub async fn select_by_type(
        pool: &Pool<Sqlite>,
        ledger_type: i64,
    ) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(
            "SELECT id, name, parent_id, image, color, ledger_type, sort_order, created_at FROM category WHERE ledger_type = ? OR ledger_type = 2 ORDER BY sort_order, id"
        )
        .bind(ledger_type)
        .fetch_all(pool)
        .await
    }

    pub async fn insert(
        pool: &Pool<Sqlite>,
        name: &str,
        parent_id: Option<i64>,
        image: &str,
        color: &str,
        ledger_type: i64,
        sort_order: i64,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "INSERT INTO category (name, parent_id, image, color, ledger_type, sort_order) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(name)
        .bind(parent_id)
        .bind(image)
        .bind(color)
        .bind(ledger_type)
        .bind(sort_order)
        .execute(pool)
        .await
    }
}

impl Tags {
    pub async fn select_list(pool: &Pool<Sqlite>) -> Result<Vec<Tags>, sqlx::Error> {
        sqlx::query_as::<_, Tags>("SELECT id, name, color, created_at FROM tags ORDER BY id")
            .fetch_all(pool)
            .await
    }

    pub async fn insert(pool: &Pool<Sqlite>, name: &str, color: &str) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO tags (name, color) VALUES (?, ?)")
            .bind(name)
            .bind(color)
            .execute(pool)
            .await
    }
}

impl Transaction {
    pub async fn insert(
        pool: &Pool<Sqlite>,
        ledger_id: i64,
        category_id: i64,
        amount: f64,
        cost_type: i64,
        comment: Option<&str>,
        transaction_time: &str,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "INSERT INTO transactions (ledger_id, category_id, amount, cost_type, comment, transaction_time) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(ledger_id)
        .bind(category_id)
        .bind(amount)
        .bind(cost_type)
        .bind(comment)
        .bind(transaction_time)
        .execute(pool)
        .await
    }

    /// 按日期查询交易记录
    pub async fn select_list_by_day(
        pool: &Pool<Sqlite>,
        date: &str,
    ) -> Result<Vec<Transaction>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT t.id, t.ledger_id, t.category_id, t.amount, t.cost_type,
                   t.comment, t.transaction_time, t.created_at, t.updated_at,
                   c.name as category_name, c.image, c.color
            FROM transactions t
            LEFT JOIN category c ON t.category_id = c.id
            WHERE date(t.transaction_time) = date(?)
            ORDER BY t.transaction_time DESC
            "#,
        )
        .bind(date)
        .fetch_all(pool)
        .await?;

        let mut transactions = Vec::new();
        for row in rows {
            transactions.push(Transaction {
                id: row.get("id"),
                ledger_id: row.get("ledger_id"),
                category_id: row.get("category_id"),
                amount: row.get("amount"),
                cost_type: row.get("cost_type"),
                comment: row.get("comment"),
                transaction_time: row.get("transaction_time"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                category_name: row.get("category_name"),
                image: row.get("image"),
                color: row.get("color"),
            });
        }
        Ok(transactions)
    }

    /// 按月份查询交易记录
    pub async fn select_list_by_month(
        pool: &Pool<Sqlite>,
        year: i64,
        month: i64,
    ) -> Result<Vec<Transaction>, sqlx::Error> {
        let start_date = format!("{:04}-{:02}-01", year, month);
        let end_date = format!("{:04}-{:02}-31", year, month);
        let rows = sqlx::query(
            r#"
            SELECT t.id, t.ledger_id, t.category_id, t.amount, t.cost_type,
                   t.comment, t.transaction_time, t.created_at, t.updated_at,
                   c.name as category_name, c.image, c.color
            FROM transactions t
            LEFT JOIN category c ON t.category_id = c.id
            WHERE date(t.transaction_time) >= date(?) AND date(t.transaction_time) <= date(?)
            ORDER BY t.transaction_time DESC
            "#,
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(pool)
        .await?;

        let mut transactions = Vec::new();
        for row in rows {
            transactions.push(Transaction {
                id: row.get("id"),
                ledger_id: row.get("ledger_id"),
                category_id: row.get("category_id"),
                amount: row.get("amount"),
                cost_type: row.get("cost_type"),
                comment: row.get("comment"),
                transaction_time: row.get("transaction_time"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                category_name: row.get("category_name"),
                image: row.get("image"),
                color: row.get("color"),
            });
        }
        Ok(transactions)
    }
}

impl Asset {
    pub async fn select_list(pool: &Pool<Sqlite>) -> Result<Vec<Asset>, sqlx::Error> {
        sqlx::query_as::<_, Asset>("SELECT * FROM assets ORDER BY id")
            .fetch_all(pool)
            .await
    }

    pub async fn insert(
        pool: &Pool<Sqlite>,
        name: &str,
        asset_type: i64,
        balance: f64,
        color: &str,
        icon: &str,
        ledger_id: Option<i64>,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "INSERT INTO assets (name, asset_type, balance, color, icon, ledger_id) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(name)
        .bind(asset_type)
        .bind(balance)
        .bind(color)
        .bind(icon)
        .bind(ledger_id)
        .execute(pool)
        .await
    }

    pub async fn update_balance(
        pool: &Pool<Sqlite>,
        id: i64,
        balance: f64,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "UPDATE assets SET balance = ?, updated_at = datetime('now', 'localtime') WHERE id = ?"
        )
        .bind(balance)
        .bind(id)
        .execute(pool)
        .await
    }
}
