use std::path::{Path, PathBuf};
use rusqlite::Connection;

use crate::{
    app::extras::Template,
    db_interface,
};


pub struct AppDBConnector {
    // a connector to interact with the database
    connection: Option<Connection>,
    path: Box<Path>,
}

impl AppDBConnector {
    pub fn new(path: PathBuf) -> AppDBConnector {
        AppDBConnector {
            connection: None,
            path: path.clone().into_boxed_path(),
        }
    }

    pub fn vec_key_to_hex(key: Vec<u8>) -> String {
        key.iter().map(|byte| format!("{:02X}", byte)).collect()
    }

    pub fn create_new_db(&mut self) {
        // creates a new database
        self.connection = Some(db_interface::create_database(&self.path));
    }

    pub fn set_db_key(&mut self, key: Vec<u8>) {
        let db_key = AppDBConnector::vec_key_to_hex(key);
        db_interface::change_password(self.connection.as_ref().unwrap(), db_key);
    }

    pub fn connect_to_db(&mut self, key: Vec<u8>) {
        // tries to connect to db if correct key (returned as bool)
        let db_key = AppDBConnector::vec_key_to_hex(key);

        if let Ok(conn) = db_interface::establish_connection(&self.path, db_key) {
            self.connection = Some(conn);
        } else {
            panic!("Failed to connect to database!")
        }
    }

    pub fn disconnect_from_db(&mut self) {
        // disconnects from db
        if let Some(conn) = self.connection.take() {
            if conn.close().is_err() {
                // ToDo: log failed connection
            }
        }
    }

    pub fn check_key_correct(&mut self, key: Vec<u8>) -> bool {
        // returns a boolean weather the entered key is correct
        let db_key = AppDBConnector::vec_key_to_hex(key);
        return db_interface::validate_key(&self.path, db_key);
    }

    pub fn get_entry_names(&self, filter: &str) -> Vec<String> {
        // gets the entry names for display (which is their id at the same time)
        db_interface::filter_for_description(self.connection.as_ref().unwrap(), filter)
    }

    pub fn get_entry(&self, name: String, key: Vec<u8>) -> (String, Vec<(String, String)>) {
        // returns a tuple with the template name the entry belongs to
        // and a list with the unencrypted entries names and the entries themselves
        db_interface::select_line(self.connection.as_ref().unwrap(), name, key)
    }

    pub fn get_all_templates(&self) -> Vec<Template>  {
        // gets all templates
        // todo here: turn templates with json_serde into objects
        db_interface::get_all_tables(self.connection.as_ref().unwrap());


        let blobs: Vec<Vec<u8>> = Vec::new();
        blobs
            .iter()
            .map(|t| serde_json::from_slice(t))
            .collect()
    }

    pub fn insert_entry(&self, template_name: String, elementes: Vec<String>, key: Vec<u8>) -> bool {
        // inserts an entry in the correct table if unique
        let description = elementes.first().unwrap().clone();
        let unique = self.check_name_available(description);

        if unique {
            db_interface::insert_entry(self.connection.as_ref().unwrap(), template_name, elementes, key)
                .expect("Failed to insert");
        }

        unique
    }

    pub fn check_name_available(&self, name: String) -> bool {
        // checks if an entry name is available or already used
        db_interface::check_name_available(self.connection.as_ref().unwrap(), name.to_string())
    }

    pub fn delete_entry(&self, name: String) {
        // tries to delete an entry if possible
        if !self.check_name_available(name.clone()) {
            db_interface::delete_entry(self.connection.as_ref().unwrap(), name);
        }
    }
}