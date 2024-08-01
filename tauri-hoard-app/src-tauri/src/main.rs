// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod account;
use account::{Account, AccountKind};
use serde_json::json;
use std::{collections::HashMap, sync::Mutex};
use tauri::Error;
use tauri::State;
use uuid::Uuid;

struct Storage {
    store: Mutex<HashMap<Uuid, String>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("Greet called from frontend [{}]", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_new_account(serialized: &str) -> Result<bool, Error> {
    let mut account: Account = serde_json::from_str(&serialized).unwrap();

    let new_id: Option<Uuid> = Some(Uuid::new_v4());
    account.id = new_id;
    println!("From Frontend // {} => {:#?}", serialized, account.id);

    Ok(true)
}

#[tauri::command]
fn get_accounts_rust() -> serde_json::Value {
    let a = Account {
        name: String::from("my checking 1234"),
        kind: AccountKind::Check,
        id: Some(Uuid::new_v4()),
    };

    let b: Account = Account {
        name: String::from("bill pay account"),
        kind: AccountKind::Check,
        id: Some(Uuid::new_v4()),
    };

    let c: Account = Account {
        name: String::from("a shoebox"),
        kind: AccountKind::Other,
        id: Some(Uuid::new_v4()),
    };

    let mut v = vec![];
    v.push(a);
    v.push(b);
    v.push(c);
    json!(v)
}

#[tauri::command]
fn storage_insert(key: &str, value: String, storage: State<Storage>) {
    // mutate the storage behind the Mutex

    // FIXME: Does this silently fail if the key is badly formed?
    let id = match Uuid::parse_str(key) {
        Ok(uuid) => uuid,
        Err(_) => return,
    };
    storage.store.lock().unwrap().insert(id, value);
}

fn main() {
    tauri::Builder::default()
        .manage(Storage {
            store: Default::default(),
        })
        .setup(|_app| Ok(()))
        .invoke_handler(tauri::generate_handler![
            greet,             // hello world
            get_accounts_rust, // get a rust Account Object as json
            save_new_account,  // handle account data sent from the UI
            storage_insert,    // Testing state management
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
