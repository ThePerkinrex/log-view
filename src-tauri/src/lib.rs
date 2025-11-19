use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use log::info;
use tauri::{Manager, State};
use tauri_plugin_dialog::DialogExt;

use crate::{
    log_read::{LogState, Record},
    settings::Settings,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod log_read;
mod settings;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn print_logs() {
//     let mut log = LogState::default();
//     log.set_file(PathBuf::from("example.json.log"));
//     for record in log.data().unwrap() {
//         ::log::info!("{record:?}")
//     }
// }

#[tauri::command]
fn get_recent_files(state: State<'_, RwLock<AppState>>) -> Vec<PathBuf> {
    info!("Reading recent files");
    state
        .read()
        .unwrap()
        .settings
        .recent
        .iter()
        .cloned()
        .collect()
}
#[tauri::command]
fn get_open_files(state: State<'_, RwLock<AppState>>) -> Vec<PathBuf> {
    info!("Reading open files");
    state.read().unwrap().open_files.keys().cloned().collect()
}

#[tauri::command]
async fn open_file(
    app: tauri::AppHandle,
    state: State<'_, RwLock<AppState>>,
) -> Result<PathBuf, ()> {
    info!("Opening file");
    let file_path = match app.dialog().file().blocking_pick_file() {
        Some(a) => a,
        None => {
            info!("NO file selected");
            return Err(());
        }
    };
    info!("Opened file");
    let path = file_path.into_path().expect("a path");

    state.write().unwrap().open(path.clone());

    Ok(path)
}

#[tauri::command]
fn close_file(
    state: State<'_, RwLock<AppState>>,
    file: PathBuf
) {

    state.write().unwrap().close(&file);

}

#[tauri::command]
fn remove_recent_file(
    state: State<'_, RwLock<AppState>>,
    file: PathBuf
) {
    state.write().unwrap().remove_recent(&file);

}

#[tauri::command]
fn get_file(state: State<'_, RwLock<AppState>>, path: PathBuf) -> Option<Vec<Record>> {
    state
        .write()
        .unwrap()
        .get(&path)
        .and_then(|x| x.data().map(|x| x.to_vec()))
}

struct AppState {
    open_files: HashMap<PathBuf, Arc<LogState>>,
    settings: Settings,
}

impl AppState {
    pub fn open(&mut self, file: PathBuf) {
        if self.settings.open(file.clone()) {
            self.open_files.insert(file.clone(), {
                let mut state = LogState::default();
                state.set_file(file);
                Arc::new(state)
            });
        }
    }

    pub fn close(&mut self, file: &Path) {
        if self.settings.close(file) {
            self.open_files.remove(file);
        }
    }

    pub fn remove_recent(&mut self, file: &Path) {
        self.close(file);
        self.settings.remove_recent(file);
    }

    pub fn get(&mut self, file: &Path) -> Option<Arc<LogState>> {
        self.open_files
            .get(file)
            .inspect(|_| {
                self.settings.open(file.to_path_buf());
            })
            .cloned()
    }
}

impl From<Settings> for AppState {
    fn from(settings: Settings) -> Self {
        Self {
            open_files: settings
                .open
                .iter()
                .cloned()
                .map(|file| {
                    (file.clone(), {
                        let mut state = LogState::default();
                        state.set_file(file);
                        Arc::new(state)
                    })
                })
                .collect(),
            settings,
        }
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        self.settings.open = self.open_files.keys().cloned().collect();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(RwLock::new(AppState::from(Settings::load())));
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Trace)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_recent_files,
            get_open_files,
            open_file,
            get_file,
            close_file,
            remove_recent_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
