// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod account;
use account::Account;
use serde_json::json;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::{fs::File, io::Write};
use tauri::Error;
use uuid::Uuid;

#[tauri::command]
fn greet(name: &str) -> String {
    println!("Greet called from frontend [{}]", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_new_account(serialized: &str) -> Result<bool, Error> {
    let mut account: Account = serde_json::from_str(&serialized).unwrap();
    let new_id = Some(Uuid::new_v4());
    account.id = new_id;
    println!("From Frontend // {} => {:#?}", serialized, account.id);

    let mut av = match read_accounts() {
        Ok(av) => av,
        Err(why) => panic!("{}", why),
    };
    av.push(account);
    let _ = write_accounts(av);

    Ok(true)
}

#[tauri::command]
fn get_accounts() -> serde_json::Value {
    let v = match read_accounts() {
        Ok(v) => json!(v),
        Err(why) => panic!("{}", why),
    };
    v
}

//fn write_accounts(accounts: Vec<Account>) -> Result<u16, std::io::Error> {
fn write_accounts(accounts: Vec<Account>) {
    let path = Path::new("accounts.json");
    let path_display = path.display();
    println!("write_accounts [{}]", path_display);

    let file = match File::create(&path) {
        Err(why) => panic!("Could not open {}: {}", path_display, why),
        Ok(file) => {
            println!("Opened![{}]", path_display);
            file
        }
    };

    let mut writer = BufWriter::new(file);
    let _ = serde_json::to_writer(&mut writer, &accounts);
    let _ = writer.flush();
    // TODO: What should we return here on success?
    // Ok(42)
}

fn read_accounts() -> Result<Vec<Account>, std::io::Error> {
    let path = Path::new("accounts.json");
    let path_display = path.display();
    println!("read_accounts [{}]", path_display);

    let file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", path_display, why),
        Ok(file) => {
            println!("Opened![{}]", path_display);
            file
        }
    };
    let reader = BufReader::new(file);
    let av: Vec<Account> = match serde_json::from_reader(reader) {
        Ok(av) => av,
        Err(why) => panic!("NOPE! [{}]", why),
    };

    /*
    let c: Account = Account {
        name: String::from("a shoebox"),
        kind: AccountKind::Other,
        id: Some(Uuid::new_v4()),
    };

    av.push(c);
     */

    for account in av.iter() {
        println!("{:?}", account);
    }
    Ok(av)
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // let _ = read_accounts();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,            // hello world
            get_accounts,     // get a rust Account Object as json
            save_new_account, // handle account data sent from the UI
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
