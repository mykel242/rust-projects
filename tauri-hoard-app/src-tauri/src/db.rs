use std::fs;
use std::path::Path;

// Check if a database file exists, and create one if it does not.
pub fn init() {
    if !db_file_exists() {
        create_db_file();
        println!("db::init // need to create db file.");
    }
}

// Create the database file.
fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    // Create the database file.
    fs::File::create(db_path).unwrap();
}

// Check whether the database file exists.
fn db_file_exists() -> bool {
    let db_path = get_db_path();
    println!("db::db_file_exists // {}", db_path);
    Path::new(&db_path).exists()
}

// Get the path where the database file should be located.
fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    let filename = "/some_path/some_file";
    /* "/.config/orion/database.sqlite" */
    println!("db::get_db_path // {}{}", home_dir.to_string_lossy(), filename);
    home_dir.to_str().unwrap().to_string() + filename
}