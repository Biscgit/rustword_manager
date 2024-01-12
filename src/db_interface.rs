use rusqlite::{Connection, params, Result};
use aes_gcm::{aead::generic_array::GenericArray};
use typenum::{U12, U32};
use crate::aes_impl::{array_from_slice, nonce_from_slice};
use crate::key_processor;

use super::aes_impl::{decrypt_aesgcm, encrypt_aesgcm, nonce_generator};
use super::base64_enc_dec::{encode_base64, decode_base64};
use super::logger;

pub fn establish_connection(db_name: &str, db_key: &str) -> Result<Connection, rusqlite::Error> {
    let db_path = db_name;
    let key = db_key;
    logger::init_logger();

    let conn = Connection::open(db_path)?;

    conn.execute_batch(&format!("PRAGMA key = '{}'", key))
        .expect("Failed to set encryption key");

    //Should be 0; default query to check if decryption failed
    let _: u32 = conn.query_row("SELECT COUNT(*) FROM sqlite_master", params![], |row| row.get(0))?;

    Ok(conn)
}

/*fn validate_connection() {
    //Placeholder; just integrate that into mainframe
    match establish_connection(db_name, db_key) {
        Ok(conn) => {change_status(&conn)}
        Err(err) => {eprintln!("Validation failed.")}
    }
}*/



// SENDING DATABASE INFORMATION TO MAINFRAME

pub fn get_all_tables(conn: &Connection) -> Vec<String> {
    let mut statement = conn.prepare("SELECT name FROM sqlite_master WHERE type='table';")
                            .expect("Failed to prepare query");
    let table_names = statement
        .query_map((), |row| row.get(0))
        .expect("Failed to execute query")
        .map(|result| result.expect("Failed to retrieve table name"))
        .collect::<Vec<String>>();
    
    let filtered_table_names: Vec<String> = table_names
        .into_iter()
        .filter(|table_name| table_name != "sqlite_sequence" && table_name != "templates")
        .collect();

    filtered_table_names
}

pub fn get_columns_from_table(conn: &Connection, table_name: &str) -> Vec<String> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table_name)).expect("Invalid table.");
    let column_names: Vec<String> = stmt.query_map([], |row| row.get(1)).expect("Failed to get column names.")
                                                                        .collect::<Result<Vec<String>, _>>()
                                                                        .expect("Failed to collect results.");
    column_names
}

pub fn get_all_columns(conn: &Connection) -> Vec<String> {
    let all_tables = get_all_tables(conn);

    let all_columns: Vec<String> = all_tables
        .iter()
        .flat_map(|table_name| get_columns_from_table(conn, table_name))
        .collect();

    all_columns
}

// IMPLEMENTING SQL COMMANDS

pub fn create_table(conn: &Connection, args_str: Vec<String>) {
    //args_str[0] is the table name, args_str[1..] is the column values for the entry
    let columns = &args_str[1..];
    let all_tables = get_all_tables(conn);
    for &item in &args_str {
        let item_string = item.to_string();
        if all_tables.contains(&item_string) {
            println!("Table {} already exists.", item);
            return;
        }
    }
    conn.execute(&format!("CREATE TABLE {} (description TEXT PRIMARY KEY, {})", args_str[0], columns.iter()
                                                                                .map(|column| format!("{} TEXT", column))
                                                                                .collect::<Vec<String>>()
                                                                                .join(", ")), params![])
                        .expect("Table name has to be unique.");

    /*
    For the future: JSON structure to be used for cleartext and hidden-text of passwords, usernames, etc. in templates table
    &str: entry_name, bool: true (visible) or false (invisible, hidden via *****, etc.)
    We probably won't have time to implement this well. But for the future, we have this building block.
    let json_map: HashMap<&str, bool> = keys.iter().zip(values.iter()).cloned().collect();
    let json_string = serde_json::to_string(&json_map).expect("Failed to convert to JSON");
    */
}

pub fn insert_entry(conn: &Connection, table_name: String, args_str: Vec<String>){
    //Take input -> Encrypt using AES -> Encode in Base64 -> Store in database
    let nonce: GenericArray<u8, U12> = nonce_generator();
    let key_gotten: Vec<u8> = key_processor::SecureStorage::get_key(&mut self); //fix
    let key = array_from_slice(&key_gotten);

    let args_aes: Vec<Vec<u8>> = args_str.into_iter().map(|s| encrypt_aesgcm(&key, &nonce, &s)).collect(); 

    let args_aes_b64: Vec<String> = args_aes.iter().map(|ciphertext| base64::encode(ciphertext)).collect();

    let args_aes_b64_string: String = format_args(args_aes_b64);

    conn.execute(&format!("INSERT INTO {} VALUES('{}, {}');", &table_name, args_aes_b64_string, base64::encode(&nonce)), params![]);
}

pub fn select_entry(conn: &Connection, table_name: String, description: String, column: String) -> String {
    //Inverse order: Decode from Base64 -> Decrypt using AES and given nonce -> return lé value
    let query_result: String = conn.query_row(&format!("SELECT {} FROM {} WHERE description = '{}';", column, table_name, description), params![], |row| row.get(0)).expect("Didnt work lol");
    let key = key_processor::SecureStorage::get_key(&mut self); //fix
    let nonce: Vec<u8> = base64::decode(conn.query_row(&format!("SELECT nonce FROM {} WHERE description = '{}';", table_name, description), params![], |row| row.get(0)).expect("")).expect("Failed decoding.");
    
    let key_usable:GenericArray<u8, U32> = array_from_slice(&key);
    let nonce_usable: GenericArray<u8, U12> = nonce_from_slice(&nonce);
    
    decrypt_aesgcm(&key_usable, &nonce_usable, &base64::decode(&query_result).expect(""))
}

pub fn delete_entry(conn: &Connection, table_name: &String, description: String) {
    conn.execute(&format!("DELETE FROM {} WHERE description = '{}'", table_name, description), params![]).expect("");
}

//HELPER FUNCTIONS

fn format_args(args_vec: Vec<String>) -> String {
    let formatted_args: String = args_vec
        .iter()
        .map(|value| format!("'{}'", value))
        .collect::<Vec<String>>()
        .join(", ");

    formatted_args
}