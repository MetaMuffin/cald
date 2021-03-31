use std::{fs::File, io::Write, sync::Mutex};

use crate::event::{Event, Tag};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref DB_EVENTS: Mutex<Vec<Event>> = Mutex::new(Vec::new());
    pub static ref DB_TAGS: Mutex<Vec<Tag>> = Mutex::new(Vec::new());
}

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    pub events: Vec<Event>,
    pub tags: Vec<Tag>,
}

pub fn load_db() {
    eprintln!("(skipping database load, as it is not implemented)")
}

pub fn save_db() {
    println!("Locking database...");
    {
        let db = (DB_EVENTS.lock().unwrap(), DB_TAGS.lock().unwrap());
        let db_copy = (
            db.0.iter().collect::<Vec<_>>(),
            db.1.iter().collect::<Vec<_>>(),
        );
        let save_path = format!(
            "{}/.cald_db",
            dirs::home_dir()
                .expect("You dont have a home directory. That's a problem!")
                .to_str()
                .expect("Your home directory path is not a string. That's a problem aswell!")
        );
        println!("Saving database to {}", save_path);
        let mut file = File::create(save_path).expect("Could not open database file");
        file.write_fmt(format_args!(
            "{}",
            serde_json::to_string(&db_copy).expect("Could not serialize database")
        ))
        .expect("Could not write to database file");
    }
    println!("Database saved!");
}
