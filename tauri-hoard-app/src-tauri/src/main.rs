// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod db;
// mod account;

use serde::{Deserialize, Serialize};
use serde_json::json;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_accounts_stub() -> String {
    let data = r#"
        { "accounts": [{ 
                    "name"  : "checking 1",
                    "type"    : "CHECK",
                    "balance" : 0.0,
                },
                {
                    "name"  : "checking 2",
                    "type"    : "CHECK",
                    "balance" : 0.0,
                },
            ]}"#;
    let s = format!("{}", data);
    println!("{}", s);
    s
}

#[derive(Debug, Serialize, Deserialize)]
enum AccountKind {
    Check = 1,
    Save = 2,
    Invest = 3,
    I529 = 529,
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    name: String,
    kind: AccountKind,
    balance: f32,
    balance_paid: f32,
}

#[tauri::command]
fn get_accounts_rust() -> serde_json::Value {
    let a = Account {
        name: "my checking 1234".to_string(),
        kind: AccountKind::Check,
        balance: 1234.56,
        balance_paid: 999.00,
    };

    json!(a)
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // db::init();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,             // hello world
            get_accounts_stub, // get hardcoded json as a string
            get_accounts_rust, // get a rust Account Object as json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
