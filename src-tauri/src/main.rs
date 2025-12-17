// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_specta::ts;

fn main() {
    // Generate TypeScript bindings for Tauri commands
    #[cfg(debug_assertions)]
    ts::export(
        specta::collect_types![code_viz_tauri::analyze_repository],
        "../src/types/bindings.ts",
    )
    .expect("Failed to generate TypeScript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            code_viz_tauri::analyze_repository
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
