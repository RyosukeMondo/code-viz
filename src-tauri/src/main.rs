// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Initialize structured logging with JSON output
    code_viz_tauri::init_logging();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            code_viz_tauri::analyze_repository
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
