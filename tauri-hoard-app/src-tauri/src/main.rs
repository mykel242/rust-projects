// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
// mod account;

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
    let s = format!("{}",data);
    println!("{}",s);
    s
}

#[derive(Debug)]
enum AccountKind {
    CHECK,
    SAVE,
    INVEST,
    I529,
}

#[derive(Debug)]
struct Account {
    name: String,
    kind: AccountKind,
    balance: f32,
    balance_paid: f32 ,   
}


fn get_accounts_rust() {

    let a = Account {
        name: "my checking 1234".to_string(),
        kind: AccountKind::CHECK,
        balance: 1234.56,
        balance_paid: 999.00
    };
    // Ok(())
}



fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            db::init();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_accounts_stub])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
