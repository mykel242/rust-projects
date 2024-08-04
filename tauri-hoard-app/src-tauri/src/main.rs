// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod account;
use account::Account;
use serde_json::json;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use tauri::Error;
use uuid::Uuid;

#[tauri::command]
fn greet(name: &str) -> String {
    println!("Greet called from frontend [{}]", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_new_account(_handle: tauri::AppHandle, serialized: &str) -> Result<bool, Error> {
    let mut account: Account = serde_json::from_str(&serialized).unwrap();
    let new_id = Some(Uuid::new_v4());
    account.id = new_id;
    println!("From Frontend // {} => {:#?}", serialized, account.id);

    let mut av = read_accounts();
    av.push(account);
    let _ = write_accounts(av);
    Ok(true)
}

#[tauri::command]
fn get_accounts_json() -> serde_json::Value {
    let v = read_accounts();
    json!(v)
}

fn write_accounts(accounts: Vec<Account>) {
    let data_path = tauri::api::path::data_dir().unwrap();
    let accounts_path = data_path
        .as_path()
        .join("xyz.dotpitch.hoard")
        .join("data")
        .join("accounts.json");
    println!("Accounts Path => [{}]", accounts_path.display());

    let file = match File::create(&accounts_path) {
        Err(why) => panic!("Could not open {:?}: {}", accounts_path, why),
        Ok(file) => {
            println!("Opened![{:?}]", accounts_path);
            file
        }
    };

    let mut writer = BufWriter::new(file);
    let _ = serde_json::to_writer(&mut writer, &accounts);
    let _ = writer.flush();
}

fn read_accounts() -> Vec<Account> {
    let accounts_path = get_accounts_path();
    let file = match File::open(&accounts_path) {
        Err(why) => panic!("Could not open {:?}: {}", accounts_path, why),
        Ok(file) => {
            println!("Opened![{:?}]", accounts_path);
            file
        }
    };
    let reader = BufReader::new(file);
    let av: Vec<Account> = match serde_json::from_reader(reader) {
        Ok(av) => av,
        Err(why) => {
            println!("NOPE! [{}]", why);
            let empty: Vec<Account> = vec![];
            empty
        }
    };

    for account in av.iter() {
        println!("{:?}", account);
    }
    av
}

fn get_accounts_path() -> PathBuf {
    let data_path = tauri::api::path::data_dir().unwrap();
    let data_dir = data_path.as_path().join("xyz.dotpitch.hoard").join("data");
    println!("Data Dir => [{}]", data_dir.display());
    create_dir_all(&data_dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    data_dir.clone().join("accounts.json")
}

fn setup_storage() {
    let accounts_path = get_accounts_path();
    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(accounts_path);
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            setup_storage();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,             // hello world
            get_accounts_json, // get a rust Account Object as json
            save_new_account,  // handle account data sent from the UI
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
