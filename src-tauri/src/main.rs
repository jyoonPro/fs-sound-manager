// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::server::AudioServer;
use std::sync::Mutex;

mod server;

struct AppState(Mutex<AudioServer>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(state: tauri::State<AppState>, name: &str) -> String {
    let mut state_guard = state.0.lock().unwrap();
    state_guard.stop_server();
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    let mut server = AudioServer::new();
    server.start_server(3000);

    tauri::Builder::default()
        .manage(AppState(Mutex::new(server)))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
