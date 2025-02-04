// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)] // 禁用本文件中所有 unused_variables 警告
#![allow(dead_code)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod game_board;
mod solvers;
mod utils;
#[tauri::command]
fn generate(difficulty: i32) -> String {
    format!("Generate function called with difficulty {}", difficulty)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
pub mod tests {
    pub mod common;  // 声明common模块
}

#[cfg(test)]
#[macro_use]
extern crate assert_matches;
