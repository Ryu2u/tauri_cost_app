use crate::error::Exception;
use crate::structs::Transaction;
use sqlx::{Pool, Sqlite};
use std::str::FromStr;
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

#[tauri::command]
pub async fn list_transactions_by_day(
    pool: State<'_, Pool<Sqlite>>,
    day: String,
) -> Result<Vec<Transaction>, Exception> {
    match chrono::DateTime::from_str(&day) {
        Ok(date_time) => match Transaction::select_list_by_day(&date_time, &pool).await {
            Ok(res) => Ok(res),
            Err(e) => Err(Exception::SqlException(format!("{}", e))),
        },
        Err(e) => Err(Exception::RuntimeException(format!("{}", e))),
    }
}
