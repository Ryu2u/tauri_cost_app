use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

#[tauri::command]
async fn greet(name: &str, app_handle: AppHandle) -> tauri::Result<String> {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
