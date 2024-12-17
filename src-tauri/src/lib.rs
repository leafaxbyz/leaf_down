// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::error::Error;
use crate::loader::book::download;

pub mod loader;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn down_book() {
    download().await.expect("TODO: panic message");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, down_book])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
