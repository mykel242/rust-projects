// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json::json;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_new_account(serialized: &str) -> bool {
    // let deserialized: Account = serde_json::from_str(&serialized).unwrap();
    println!("From Frontend // {}", serialized);
    true
}

#[derive(Debug, Serialize, Deserialize)]
enum AccountKind {
    Check = 1,
    Save = 2,
    Invest = 3,
    I529 = 4,
    Other = 5,
    Debt = 6,
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
        name: String::from("my checking 1234"),
        kind: AccountKind::Check,
        balance: 1234.56,
        balance_paid: 0.0,
    };

    let b: Account = Account {
        name: String::from("bill pay account"),
        kind: AccountKind::Check,
        balance: 987.65,
        balance_paid: 0.0,
    };

    let c: Account = Account {
        name: String::from("a shoebox"),
        kind: AccountKind::Other,
        balance: 17.12,
        balance_paid: 0.0,
    };

    let mut v = vec![];
    v.push(a);
    v.push(b);
    v.push(c);
    json!(v)
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // db::init();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,             // hello world
            get_accounts_rust, // get a rust Account Object as json
            save_new_account,  // handle account data sent from the UI
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
