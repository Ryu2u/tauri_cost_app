use tauri::AppHandle;
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
