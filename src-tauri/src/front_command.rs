use tauri::{Manager, Runtime};

#[tauri::command]
pub async fn close_splashscreen<R: Runtime>(window: tauri::Window<R>) {
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }

    window.get_window("main").unwrap().show().unwrap();
    window.get_window("main").unwrap().set_focus().unwrap();
}