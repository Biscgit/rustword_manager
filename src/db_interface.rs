use rusqlite::{Connection, params, Result};
use aes_gcm::aead::generic_array::GenericArray;
use typenum::{U12, U32};
use chrono::prelude::Utc;
use crate::aes_impl::{u12_from_slice, u32_from_slice};
use crate::key_processor;

use super::aes_impl::{decrypt_aesgcm, encrypt_aesgcm, nonce_generator};
use super::base64_enc_dec::{encode_base64, decode_base64};
use super::logger;

pub fn establish_connection(db_name: &str, db_key: &str) -> Result<Connection, rusqlite::Error> {
    let db_path = db_name;
    let key = db_key; //NEED SECURE STORAGE HERE
    logger::init_logger(&format!("RustwordManager_{}.log", Utc::now().format("%Y%m%d_%H%M%S"))); //PUT THIS INTO main.rs

    let conn = Connection::open(db_path)?;

    conn.execute_batch(&format!("PRAGMA key = '{}'", key))
        .expect("Failed to set encryption key");

    //Should be 0; default query to check if decryption failed; writing to _ is necessary because of row.get()
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
        .map(|result| decode_base64(result.expect("Failed to retrieve table name")))
        .collect::<Vec<String>>();
    
    let filtered_table_names: Vec<String> = table_names
        .into_iter()
        .filter(|table_name| decode_base64(table_name.to_string()) != "sqlite_sequence" && decode_base64(table_name.to_string()) != "templates" && decode_base64(table_name.to_string()) != "nonce")
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

pub fn decode_vec_string_b64(encoded_vec: Vec<String>) -> Vec<String> {
    let decoded_vec: Vec<String> = encoded_vec
        .iter()
        .map(|encoded_column| decode_base64(encoded_column.to_string()))
        .collect();
    decoded_vec
}

pub fn get_all_columns_total(conn: &Connection) -> Vec<String> {
    let all_tables = get_all_tables(conn);

    let all_columns: Vec<String> = all_tables
        .iter()
        .flat_map(|table_name| get_columns_from_table(conn, table_name))
        .collect();

    all_columns
}

pub fn filter_for_description(conn: &Connection, input: &str) -> Vec<String> { // %<Word>% is a before-and-after wildcard in SQL.
    let mut return_vec: Vec<String> = vec![];

    let all_tables: Vec<String> = get_all_tables(conn);
    let mut all_descriptions: Vec<String> = vec![];
    for table in &all_tables {
        if let Ok(description) = conn.query_row(
            &format!("SELECT description FROM {}", table),
            params![],
            |row| row.get(0),
        ) {
            all_descriptions.push(decode_base64(description));
        }
    }
    for desc in all_descriptions.iter() {
        if desc.contains(input) {
            return_vec.push(desc.to_string());
        }
    }
        
    return_vec

}

// IMPLEMENTING SQL COMMANDS

pub fn create_table(conn: &Connection, table_name: String, columns: Vec<String>) -> Result<()> {
    conn.execute(&format!("CREATE TABLE {} (description TEXT, {}, nonce TEXT UNIQUE)", base64::encode(table_name), columns.iter()
                                                                                .map(|column| format!("{} TEXT", base64::encode(column)))
                                                                                .collect::<Vec<String>>()
                                                                                .join(", ")), params![])?;
    Ok(())

    /*
    For the future: JSON structure to be used for cleartext and hidden-text of passwords, usernames, etc. in templates table
    &str: entry_name, bool: true (visible) or false (invisible, hidden via *****, etc.)
    We probably won't have time to implement this well. But for the future, we have this building block.
    let json_map: HashMap<&str, bool> = keys.iter().zip(values.iter()).cloned().collect();
    let json_string = serde_json::to_string(&json_map).expect("Failed to convert to JSON");
    */
}

pub fn insert_entry(conn: &Connection, table_name: String, args_str: Vec<String>, key: Vec<u8>) -> Result<()> {
    //Take input -> Encrypt using AES -> Encode in Base64 -> Store in 
    let mut nonce: GenericArray<u8, U12>  = GenericArray::default();
    loop {
        nonce = nonce_generator();
        if conn.query_row(&format!("SELECT 1 FROM nonces WHERE nonce = '{}'", base64::encode(&nonce)), params![], |_| Ok(1))
        .is_err() {
            //This query ensures that the generates nonce is unique; the odds of generating two same random 96 bit numbers are low, but never zero!
            conn.execute(&format!("INSERT INTO nonces VALUES('{}')", base64::encode(&nonce)), params![]).expect("Something went wrong.");
            break;
        }
    }
    let key_as_array = u32_from_slice(&key);

    let args_aes: Vec<Vec<u8>> = args_str.into_iter().map(|s| encrypt_aesgcm(&key_as_array, &nonce, &s)).collect(); 

    let args_aes_b64: Vec<String> = args_aes.iter().map(|ciphertext| base64::encode(ciphertext)).collect();

    let args_aes_b64_string: String = format_args(args_aes_b64); //add ' ', around all entries

    conn.execute(&format!("INSERT INTO {} VALUES('{}, {}');", &table_name, args_aes_b64_string, base64::encode(&nonce)), params![])?;

    Ok(())
}

pub fn select_entry(conn: &Connection, table_name: String, description: String, column: String, key: Vec<u8>) -> String {
    //Inverse order: Decode from Base64 -> Decrypt using AES and given nonce -> return lé value
    let query_result: String = conn.query_row(&format!("SELECT {} FROM {} WHERE description = '{}';", column, table_name, description), params![], |row| row.get(0)).expect("Didnt work lol");
    let stmt: String = conn.query_row(&format!("SELECT nonce FROM {} WHERE description = '{}';", table_name, description), params![], |row| row.get(0)).expect("");
    let nonce: Vec<u8> = base64::decode(stmt).expect("Failed decoding.");
    
    let key_usable:GenericArray<u8, U32> = u32_from_slice(&key);
    let nonce_usable: GenericArray<u8, U12> = u12_from_slice(&nonce);
    
    decrypt_aesgcm(&key_usable, &nonce_usable, &base64::decode(&query_result).expect(""))
}

pub fn delete_entry(conn: &Connection, table_name: &String, description: String) -> Result<()> {
    conn.execute(&format!("DELETE FROM {} WHERE description = '{}'", table_name, description), params![])?;
    Ok(())
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