use std::{
    fs::{create_dir_all, File},
    process::exit,
};

use chrono::Local;
use dirs::home_dir;

const LOG_DIRECTORY: &'static str = ".local/share/todui/logs";
const LOG_FILENAME_FORMAT: &'static str = "%Y-%m-%d_%H-%M-%S.log";

pub fn initialize_log_file() -> File {
    let directory = match home_dir() {
        Some(dir) => dir,
        None => {
            println!("Failed to get home directory.");
            exit(-1);
        }
    }
    .join(LOG_DIRECTORY);

    if !directory.exists() {
        create_dir_all(&directory).expect("");
    }

    File::create(directory.join(Local::now().format(LOG_FILENAME_FORMAT).to_string())).unwrap()
}
