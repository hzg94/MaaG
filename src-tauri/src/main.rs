// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod front_command;
mod adb_command;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![front_command::close_splashscreen,adb_command::list_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
