use std::path::PathBuf;

use crate::log_read::LogState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod log_read;
mod settings;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn print_logs() {
    let mut log = LogState::default();
    log.set_file(PathBuf::from("example.json.log"));
    for record in log.data().unwrap() {
        ::log::info!("{record:?}")
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Trace)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, print_logs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
