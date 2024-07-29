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
    // -- Write some account objects to a json file --
    // let accounts = vec![
    //     Account {
    //         name: "Checking #222".to_string(),
    //         balance: 999.00,
    //         kind: AccountKind::Check,
    //     },
    //     Account {
    //         name: "Slush #1234".to_string(),
    //         balance: 1999.00,
    //         kind: AccountKind::Save,
    //     },
    // ];

    // let json = serde_json::to_string(&accounts).expect("Serialization broked.");
    // println!("Serialized:\n {}", json);

    // let out_file = File::create("outfile.json")?;
    // let mut writer = BufWriter::new(out_file);
    // serde_json::to_writer(&mut writer, &accounts)?;
    // writer.flush()?;

    // -- Read some account objects from a json file --
    let in_file = File::open("infile.json")?;
    let mut reader = BufReader::new(in_file);
    let read_accounts: Vec<Account> = serde_json::from_reader(reader)?;
    println!("Read Accounts;\n {:#?}", read_accounts);

    for account in read_accounts.iter() {
        println!("[{}] => ${}", account.name, account.balance);
    }

    Ok(())
}
