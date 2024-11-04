use chrono::Local;
use dirs::home_dir;
use std::{
    fs::{self, create_dir_all, File},
    path::PathBuf,
    process::exit,
};

const LOG_DIRECTORY: &str = ".local/share/todui/logs";
const LOG_FILENAME_FORMAT: &str = "%Y-%m-%d_%H-%M-%S.log";
const LOG_FILE_COUNT_LIMIT: usize = 10;

pub fn initialize_log_file() -> File {
    let directory = get_log_directory();

    clear_log_directory(&directory);

    let file = directory.join(Local::now().format(LOG_FILENAME_FORMAT).to_string());
    File::create(file).unwrap()
}

fn get_log_directory() -> PathBuf {
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

    directory
}

fn clear_log_directory(directory: &PathBuf) {
    let file_count = match fs::read_dir(directory) {
        Ok(paths) => paths
            .into_iter()
            .filter(|path| match path {
                Ok(path) => path.path().is_file(),
                Err(_) => false,
            })
            .count(),
        Err(_) => 0,
    };

    if file_count > LOG_FILE_COUNT_LIMIT {
        fs::remove_dir_all(directory).unwrap();
        fs::create_dir(directory).unwrap();
    }
}
