use std::io::{BufReader, BufWriter};
use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};
// use serde_json::json;

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
}

fn main() -> Result<(), std::io::Error> {
    let path = "infile.json".to_string();
    // -- Read some account objects from a json file --
    let in_file = File::open(&path)?;
    let reader = BufReader::new(in_file);
    let mut accounts: Vec<Account> = serde_json::from_reader(reader)?;
    println!("Read Accounts;\n {:#?}", accounts);

    for account in accounts.iter() {
        println!("[{}] => ${}", account.name, account.balance);
    }

    // -- Write some account objects to a json file --
    let new_account = Account {
        name: "a new one".to_string(),
        balance: 0.0,
        kind: AccountKind::Other,
    };

    // Append the new Account
    accounts.push(new_account);
    // let json = serde_json::to_string(&accounts).expect("Serialization broked.");
    // println!("Serialized:\n {}", json);

    let out_file = File::create(&path)?;
    let mut writer = BufWriter::new(out_file);
    serde_json::to_writer(&mut writer, &accounts)?;
    writer.flush()?;
    Ok(())
}
