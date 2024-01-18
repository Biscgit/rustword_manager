use std::path::PathBuf;
use rusqlite::Connection;

use crate::db_interface;


pub struct AppDBConnector {
    // a connector to interact with the database
    connection: Option<Connection>,
}

impl AppDBConnector {
    pub fn new() -> AppDBConnector {
        AppDBConnector { connection: None }
    }

    pub fn connect_to_db(&mut self, path: PathBuf, key: Vec<u8>) -> bool {
        // tries to connect to db if correct key
        let db_key: String = key.iter().map(|byte| format!("{:02X}", byte)).collect();

        let conn = db_interface::establish_connection(path, db_key);
        if let Ok(conn) = conn {
            self.connection = Some(conn);
        } else {
            //panic!("Error establishing connection");
        }

        conn.is_ok()
    }

    pub fn disconnect_from_db(&mut self) {
        // disconnects from db
        if let Some(conn) = self.connection.take() {
            if conn.close().is_err() {
                // ToDo: log failed connection
            }
        }
    }

    //pub fn check_key_correct(key: &str) -> bool { Should not be necessary; is implemented in connect_to_db()
    // checks if the entered key is correct
    //}

    pub fn get_entry_names(&self, filter: &str) -> Vec<String> {
        // gets the entry names for display (which is their id at the same time)
        db_interface::filter_for_description(self.connection.as_ref().unwrap(), filter)
    }

    pub fn get_entry(&self, name: String, key: Vec<u8>) -> (String, Vec<(String, String)>) {
        // returns a tuple with the template name the entry belongs to
        // and a list with the unencrypted entries names and the entries themselves
        db_interface::select_line(self.connection.as_ref().unwrap(), name, key)
    }

    pub fn get_all_templates(&self) -> Vec<String> {
        // gets all templates
        // todo here: turn templates with json_serde into objects
        db_interface::get_all_tables(self.connection.as_ref().unwrap())
    }

    pub fn insert_entry(&self, template_name: String, elementes: Vec<String>, key: Vec<u8>) {
        // inserts an entry in the correct table if unique
        let description: String = elementes.first().unwrap().to_string();
        if self.check_name_available(description) {
            db_interface::insert_entry(self.connection.as_ref().unwrap(), template_name, elementes, key);
        }
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