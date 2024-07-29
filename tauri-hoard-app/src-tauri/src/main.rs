// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use serde::{Deserialize, Serialize};
// use account::{Account, AccountKind};
use serde_json::json;
mod account;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_new_account(serialized: &str) -> bool {
    let deserialized: account::Account = serde_json::from_str(&serialized).unwrap();
    println!("From Frontend // {}", serialized);
    println!(
        "Deserialized;\n[{}] => {}",
        deserialized.name, deserialized.balance
    );
    true
}

#[tauri::command]
fn get_accounts_rust() -> serde_json::Value {
    let a = account::Account {
        name: String::from("my checking 1234"),
        kind: account::AccountKind::Check,
        balance: 1234.56,
    };

    let b: account::Account = account::Account {
        name: String::from("bill pay account"),
        kind: account::AccountKind::Check,
        balance: 987.65,
    };

    let c: account::Account = account::Account {
        name: String::from("a shoebox"),
        kind: account::AccountKind::Other,
        balance: 17.12,
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
