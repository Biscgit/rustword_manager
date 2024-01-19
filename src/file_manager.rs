use chrono::Utc;
use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};


const PATH: [&str; 1] = ["RustwordManager"];
const DB_NAME: &str = "passwords.sqlite3";

pub struct FileManager {
    // interacts with the filesystem
    pub filepath: PathBuf,
    pub salt: Option<[u8; 16]>,
}

impl FileManager {
    pub fn new() -> FileManager {
        // create new file manager that holds path
        FileManager {
            filepath: FileManager::get_db_path(),
            salt: None,
        }
    }

    fn get_db_path() -> PathBuf {
        // gets the db path if not exist
        let mut home_dir = dirs::home_dir().expect("Failed to open Home directory");
        for part in PATH {
            home_dir.push(part);
        }

        home_dir
    }

    pub fn create_path(&self) -> io::Result<PathBuf> {
        // creates the db path if not exists
        fs::create_dir_all(self.filepath.as_path())?;

        let mut filepath = self.filepath.clone();
        filepath.push(DB_NAME);

        Ok(filepath)
    }

    pub fn check_db_exist(&self) -> bool {
        // returns a boolean weather the Database file exists
        let mut filepath = self.filepath.clone();
        filepath.push(DB_NAME);

        if let Ok(metadata) = fs::metadata(filepath) {
            return metadata.is_file();
        }

        false
    }

    pub fn check_lock_set(&self) -> io::Result<bool> {
        // sets a file-lock if instance is running
        // returns weather an instance is already running

        let mut filepath = self.filepath.clone();
        filepath.push("lock");

        // checks if file exists
        let mut exists = false;
        if let Ok(metadata) = fs::metadata(filepath.as_path()) {
            if metadata.is_file() {
                exists = true;
            }
        }

        // locks file
        if !exists {
            File::create(filepath.as_path())?;
        }

        Ok(exists)
    }

    pub fn release_file_lock(&self) -> io::Result<()> {
        // release the lock of the instance

        let mut filepath = self.filepath.clone();
        filepath.push("lock");

        if fs::metadata(filepath.as_path()).is_ok() {
            // Attempt to remove the file
            fs::remove_file(filepath.as_path())?;
        }

        Ok(())
    }

    pub fn get_salt(&mut self) -> io::Result<[u8; 16]> {
        // sqlcipher stores a random salt as the first 16 bytes of a file

        if let Some(salt) = self.salt {
            Ok(salt)
        } else {
            let mut file = fs::File::open(self.create_path().unwrap())?;

            let mut buf = [0; 16];
            file.read_exact(&mut buf)?;

            self.salt = Some(buf);
            Ok(buf)
        }
    }

    pub fn get_logger_path(&self) -> PathBuf {
        // creates a new logging path

        let mut logging_path = self.filepath.clone();

        logging_path.push("logs");
        logging_path.push(
            &format!("RWManager_{}.log", Utc::now().format("%Y%m%d_%H%M%S"))
        );

        logging_path
    }
}
