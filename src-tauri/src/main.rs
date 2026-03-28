// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() {
    if let Err(e) = rust_tauri2_demo_lib::run().await {
        eprintln!("应用启动失败: {}", e);
        std::process::exit(1);
    }
}
