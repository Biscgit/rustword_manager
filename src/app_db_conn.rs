use rusqlite::Connection;
use crate::db_interface;

const PATH: [&str; 2] = ["RustwordManager", "passwords.db"];

struct AppDBConnector {
    // a connector to interact with the database
    connection: Option<Connection>,
}

impl AppDBConnector {
    pub fn new() -> AppDBConnector {
        AppDBConnector { connection: None }
    }

    pub fn connect_to_db(&mut self, key: Vec<u8>) {
        // tries to connect to db if correct key
        let db_key = key.iter().map(|byte| format!("{:02X}", byte)).collect();

        if self.key_valid(db_key) {
            self.connection = Some(db_interface::establish_connection("", db_key).unwrap());
        }
    }

    pub fn disconnect_from_db(&mut self) {
        // disconnects from db
        ...
        self.connection = None;
    }

    pub fn check_key_correct(key: &str) -> bool {
        // checks if the entered key is correct
    }

    pub fn get_entry_names(&self, filter: Option<&str>) -> Vec<String> {
        // gets the entry names for display (which is their id at the same time)
        ...
    }

    pub fn get_entry(&self, name: String) -> (String, Vec<(String, String)>) {
        // returns a tuple with the template name the entry belongs to
        // and a list with the unencrypted entries names and the entries themselves
    }

    pub fn get_all_templates(&self) -> Vec<String> {
        // gets all templates
        // todo here: turn templates with json_serde into objects
    }

    pub fn insert_entry(&self, template_name: String, elementes: Vec<String>) {
        // inserts an entry in the correct table if unique
        if self.check_name_available(elementes.first().unwrap()) {}
    }

    pub fn check_name_available(&self, name: &String) -> bool {
        // checks if an entry name is available or already used
    }

    pub fn delete_entry(&self, name: String) {
        // tries to delete an entry if possible
        if self.check_name_available(&name) {}
    }
}