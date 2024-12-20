// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod game_board;

#[tauri::command]
fn generate(difficulty: i32) -> String{
    format!("Generate function called with difficulty {}",difficulty)
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
