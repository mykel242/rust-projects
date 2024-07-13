// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn get_accounts() -> String {

//     format!("{{  \
//             accounts:[ \
//             {{ name: \"checking\",type: \"CHECK\",balance: 0.0,}}, \
//             ]}}")
// }

mod db;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            db::init();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        //.invoke_handler(tauri::generate_handler![get_accounts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
