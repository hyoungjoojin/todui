use std::{
    fs::{create_dir_all, File},
    process::exit,
};

use chrono::Local;
use dirs::home_dir;

pub fn initialize_log_file() -> File {
    let directory = match home_dir() {
        Some(dir) => dir,
        None => {
            println!("Failed to get home directory.");
            exit(-1);
        }
    }
    .join(".local")
    .join("share")
    .join("todui")
    .join("logs");

    if !directory.exists() {
        create_dir_all(&directory).expect("");
    }

    File::create(directory.join(Local::now().format("%Y-%m-%d_%H-%M-%S.log").to_string())).unwrap()
}
