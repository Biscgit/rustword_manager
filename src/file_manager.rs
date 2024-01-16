use std::{fs, io::{self, Read}};
use std::path::PathBuf;
use dirs;

const PATH: [&str; 2] = ["RustwordManager", "database.db"];


pub struct FileManager {
    filepath: PathBuf,
}

impl FileManager {
    pub fn new() -> FileManager {
        // create new file manager that holds path
        FileManager { filepath: FileManager::get_db_path() }
    }

    fn get_db_path() -> PathBuf {
        // gets the db path if not exist
        let mut home_dir = dirs::home_dir().expect("Failed to open Home directory");
        for part in PATH {
            home_dir.push(part);
        }

        home_dir
    }

    pub fn create_path(&self) -> io::Result<()> {
        // creates the db path if not exists
        fs::create_dir_all(self.filepath.as_path())
    }

    pub fn check_db_exist(&self) -> bool {
        // returns a boolean weather the Database file exists
        if let Ok(metadata) = fs::metadata(self.filepath.as_path()) {
            return metadata.is_file();
        }
        false
    }

    pub fn get_salt(&self) -> io::Result<[u8; 16]> {
        // sqlcipher stores a random salt as the first 16 bytes of a file
        let mut file = fs::File::open(self.filepath.as_path())?;

        let mut buf = [0; 16];
        file.read_exact(&mut buf)?;

        Ok(buf)
    }
}