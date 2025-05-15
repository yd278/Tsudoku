#![allow(unused_variables)] // 禁用本文件中所有 unused_variables 警告
#![allow(dead_code)]

mod game_board;
mod solvers;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![generate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
fn generate(difficulty: i32) -> String {
    let cell = game_board::Cell::Printed(3);
    format!("Generate function called with difficulty {}", difficulty)
}

#[cfg(test)]
pub mod tests {
    pub mod common; // 声明common模块
}
