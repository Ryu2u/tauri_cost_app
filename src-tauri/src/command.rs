use crate::error::Exception;
use crate::structs::{Asset, Category, Ledger, Tags, Transaction};
use sqlx::{Pool, Sqlite};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

#[tauri::command]
pub async fn greet(name: &str, app_handle: AppHandle) -> tauri::Result<String> {
    let _answer = app_handle
        .dialog()
        .message(format!("Hello, {}!", name))
        .title(format!("Hello {}", name))
        .buttons(MessageDialogButtons::OkCancelCustom(
            "OK".to_string(),
            "CANCEL".to_string(),
        ))
        .blocking_show();
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

// ==================== 账本相关 ====================

#[tauri::command]
pub async fn list_ledgers(pool: State<'_, Pool<Sqlite>>) -> Result<Vec<Ledger>, Exception> {
    Ledger::select_list(&pool)
        .await
        .map_err(|e| Exception::SqlException(format!("{}", e)))
}

// ==================== 分类相关 ====================

#[tauri::command]
pub async fn list_categories(pool: State<'_, Pool<Sqlite>>) -> Result<Vec<Category>, Exception> {
    Category::select_list(&pool)
        .await
        .map_err(|e| Exception::SqlException(format!("{}", e)))
}

#[tauri::command]
pub async fn list_categories_by_type(
    pool: State<'_, Pool<Sqlite>>,
    ledger_type: i64,
) -> Result<Vec<Category>, Exception> {
    Category::select_by_type(&pool, ledger_type)
        .await
        .map_err(|e| Exception::SqlException(format!("{}", e)))
}

// ==================== 标签相关 ====================

#[tauri::command]
pub async fn list_tags(pool: State<'_, Pool<Sqlite>>) -> Result<Vec<Tags>, Exception> {
    Tags::select_list(&pool)
        .await
        .map_err(|e| Exception::SqlException(format!("{}", e)))
}

// ==================== 交易相关 ====================

#[tauri::command]
pub async fn add_transaction(
    pool: State<'_, Pool<Sqlite>>,
    ledger_id: i64,
    category_id: i64,
    amount: f64,
    cost_type: i64,
    comment: Option<String>,
    transaction_time: String,
) -> Result<i64, Exception> {
    let result = Transaction::insert(
        &pool,
        ledger_id,
        category_id,
        amount,
        cost_type,
        comment.as_deref(),
        &transaction_time,
    )
    .await
    .map_err(|e| Exception::SqlException(format!("{}", e)))?;

    Ok(result.last_insert_rowid())
}

#[tauri::command]
pub async fn list_transactions_by_day(
    pool: State<'_, Pool<Sqlite>>,
    day: String,
) -> Result<Vec<Transaction>, Exception> {
    match chrono::NaiveDate::parse_from_str(&day, "%Y-%m-%d") {
        Ok(date) => match Transaction::select_list_by_day(&pool, &date.format("%Y-%m-%d").to_string()).await {
            Ok(res) => Ok(res),
            Err(e) => Err(Exception::SqlException(format!("{}", e))),
        },
        Err(e) => Err(Exception::RuntimeException(format!("{}", e))),
    }
}

#[tauri::command]
pub async fn list_transactions_by_month(
    pool: State<'_, Pool<Sqlite>>,
    year: i64,
    month: i64,
) -> Result<Vec<Transaction>, Exception> {
    Transaction::select_list_by_month(&pool, year, month)
        .await
        .map_err(|e| Exception::SqlException(format!("{}", e)))
}

// ==================== 资产相关 ====================

#[tauri::command]
pub async fn list_assets(pool: State<'_, Pool<Sqlite>>) -> Result<Vec<Asset>, Exception> {
    Asset::select_list(&pool)
        .await
        .map_err(|e| Exception::SqlException(format!("{}", e)))
}
