use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{FromRow, Pool, Sqlite};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ledger {
    pub id: i64,
    pub ledger_name: String,
    pub ledger_type: i64,
    pub ledger_image: String,
    pub create_time: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tags {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TransactionTgs {
    pub transaction_id: i64,
    pub tag_id: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i64,
    pub ledger_id: i64,
    pub category_id: i64,
    pub amount: i64,
    pub comment: Option<String>,
    pub transaction_time: String,
    pub created_at: String,
    // field not exists
    #[sqlx(skip)]
    pub category_name: Option<String>,
    #[sqlx(skip)]
    pub image: Option<String>,
}

impl Ledger {
    pub fn new(ledger_name: String, ledger_type: i64) -> Self {
        Ledger {
            id: 0,
            ledger_name,
            ledger_type,
            ledger_image: String::new(),
            create_time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    pub async fn select_list(pool: &Pool<Sqlite>) -> Result<Vec<Ledger>, sqlx::Error> {
        sqlx::query_as!(Ledger, "select * from ledger")
            .fetch_all(pool)
            .await
    }

    pub async fn insert(&self, pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
        let ledger_name = self.ledger_name.clone();
        let ledger_type = self.ledger_type;
        let ledger_image = self.ledger_image.clone();
        let create_time = self.create_time.clone();
        sqlx::query!(
            "INSERT INTO ledger (ledger_name, ledger_type,ledger_image, create_time) values(?, ?, ?, ?)",
            ledger_name,
            ledger_type,
            ledger_image,
            create_time
        ).execute(pool).await
    }
}

impl Category {
    pub fn new(name: String, parent_id: Option<i64>, image: String) -> Self {
        Category {
            id: 0,
            name,
            parent_id,
            image,
        }
    }

    /// 添加分类
    pub async fn insert(&self, pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
        let name = self.name.clone();
        let parent_id = self.parent_id.clone();
        let image = self.image.clone();
        sqlx::query!(
            "INSERT INTO category (name, parent_id, image) VALUES (?, ?, ?)",
            name,
            parent_id,
            image
        )
        .execute(pool)
        .await
    }

    /// 查询所有的分类
    pub async fn select_list(pool: &Pool<Sqlite>) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as!(Category, "select * from category order by id")
            .fetch_all(pool)
            .await
    }
}

impl Tags {
    pub fn new(name: String) -> Self {
        Tags { id: 0, name }
    }

    pub async fn insert(&self, pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
        let name = self.name.clone();
        sqlx::query!("INSERT INTO tags (name) VALUES (?)", name)
            .execute(pool)
            .await
    }

    pub async fn select_list(pool: &Pool<Sqlite>) -> Result<Vec<Tags>, sqlx::Error> {
        sqlx::query_as!(Tags, "SELECT id, name FROM tags ORDER BY id",)
            .fetch_all(pool)
            .await
    }
}

impl Transaction {
    pub fn new(
        ledger_id: i64,
        category_id: i64,
        amount: i64,
        transaction_time: String,
        comment: Option<String>,
    ) -> Self {
        Transaction {
            id: 0,
            ledger_id,
            category_id,
            amount,
            comment,
            transaction_time,
            created_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            category_name: None,
            image: None,
        }
    }

    /// 添加新的交易记录
    pub async fn insert(&self, pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
        let ledger_id = self.ledger_id;
        let category_id = self.category_id;
        let amount = self.amount;
        let transaction_time = self.transaction_time.clone();
        let comment = self.comment.clone();
        let create_at = self.created_at.clone();
        sqlx::query!(
            "INSERT INTO transactions (ledger_id, category_id, amount, comment, transaction_time, created_at) VALUES (?, ?, ?, ?, ?, ?)",
            ledger_id,
            category_id,
            amount,
            comment,
            transaction_time,
            create_at
        ).execute(pool).await
    }

    /// 查询当天的交易记录
    pub async fn select_list_by_day(
        day: &chrono::DateTime<Local>,
        pool: &Pool<Sqlite>,
    ) -> Result<Vec<Transaction>, sqlx::Error> {
        let day_str = day.format("%Y-%m-%d").to_string();
        sqlx::query_as::<_, Transaction>(
            "select id,ledger_id,category_id,amount,comment,transaction_time,created_at from transactions where date(transaction_time) = date(?)  order by transaction_time desc",
        ).bind(day_str).fetch_all(pool).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::get_connection;

    #[tokio::test]
    async fn test_category_save() {
        let con = get_connection().await.expect("获取数据库连接失败");
        let mut vec = vec![];
        let item1 = Category::new("餐饮".to_string(), None, "1".to_string());
        let item2 = Category::new("服饰".to_string(), None, "2".to_string());
        let item3 = Category::new("日用".to_string(), None, "3".to_string());
        let item4 = Category::new("数码".to_string(), None, "4".to_string());
        let item5 = Category::new("应用软件".to_string(), None, "5".to_string());
        let item6 = Category::new("住房".to_string(), None, "6".to_string());
        let item7 = Category::new("交通".to_string(), None, "7".to_string());
        let item8 = Category::new("娱乐".to_string(), None, "8".to_string());
        vec.push(item1);
        vec.push(item2);
        vec.push(item3);
        vec.push(item4);
        vec.push(item5);
        vec.push(item6);
        vec.push(item7);
        vec.push(item8);
        for x in vec {
            Category::insert(&x, &con).await.expect("插入分类数据失败");
        }
    }

    #[tokio::test]
    async fn test_transaction_list() {
        let now = Local::now();
        let con = get_connection().await.expect("获取数据库连接失败");
        let res = Transaction::select_list_by_day(&now, &con)
            .await
            .expect("查询当日交易记录失败");
        res.iter().for_each(|v| println!("{:?}", v));
    }
}
