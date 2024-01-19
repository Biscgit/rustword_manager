use std::path::Path;
use crate::aes_impl::{decrypt_aesgcm, encrypt_aesgcm, nonce_generator, u12_from_slice, u32_from_slice};
use crate::base64_enc_dec::{encode_base64, decode_base64};

use rusqlite::{Connection, params, Result};
use aes_gcm::aead::generic_array::GenericArray;
use typenum::{U12, U32};

use crate::password::generate_char_only_password;

const SQL_INITIALIZE: &str = include_str!("./sql/initiate.sql");


pub fn create_database(path: &Box<Path>) -> Connection {
    //Used when first creating a file; returns connection
    let conn: Connection = Connection::open(path)
        .expect("Failed to create db");

    // sent temporary key for protection while initializing
    let temp_key = generate_char_only_password(32);
    conn.execute_batch(&format!("PRAGMA key = '{}'", temp_key))
        .expect("Failed to set key");

    // fill database with default config from .sql file
    conn.execute_batch(SQL_INITIALIZE)
        .expect("Failed to initialize database");

    return conn;
}

pub fn change_password(conn: &Connection, new_key: String) {
    //Only called during database creation; sets a new password for the database.
    conn.execute_batch(&format!("PRAGMA rekey = '{}'", new_key))
        .expect("Failed to change key");
}

pub fn establish_connection(db_path: &Box<Path>, db_key: String) -> Result<Connection, rusqlite::Error> {
    //Connect to database. Will return an Error if it didn't work.
    let conn = Connection::open(db_path)?;

    conn.execute_batch(&format!("PRAGMA key = '{}'", db_key))
        .expect("Failed to set encryption key");

    //If the database was not decrypted successfully, this operation will fail and throw an Error. Else, it will work.
    let _: u32 = conn.query_row("SELECT COUNT(*) FROM sqlite_master", params![], |row| row.get(0))?;

    Ok(conn)
}

pub fn validate_key(db_path: &Box<Path>, db_key: String) -> bool {
    // Returns true if given key is valid.
    let key = db_key;
    //logger::init_logger(&format!("RustwordManager_{}.log", Utc::now().format("%Y%m%d_%H%M%S"))); //PUT THIS INTO main.rs

    let conn = Connection::open(db_path).expect("");

    conn.execute_batch(&format!("PRAGMA key = '{}'", key))
        .expect("Failed to set encryption key");

    //Should be 0; default query to check if decryption failed; writing to _ is necessary because of row.get()
    conn.query_row("SELECT COUNT(*) FROM sqlite_master", params![], |row| row.get::<usize, usize>(0)).is_ok()
}

// SENDING DATABASE INFORMATION TO MAINFRAME

pub fn get_all_tables(conn: &Connection) -> Vec<String> {
    // Returns all tables created by a user; return-values are NOT base64-encoded.
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table';").expect("");
    let table_names: Vec<String> = stmt.query_map([], |row| row.get(0)).expect("")
        .collect::<Result<Vec<String>, >>().expect("");

    let filtered_table_names: Vec<String> = table_names
        .into_iter()
        .filter(|table_name| table_name.to_string() != "sqlite_sequence" && table_name.to_string() != "templates" && table_name.to_string() != "nonces" && table_name.to_string() != "descriptions") //Exclude backend-only tables
        .map(|table_name| decode_base64(table_name))
        .collect();

    filtered_table_names
}

pub fn get_columns_from_table(conn: &Connection, table_name: &str) -> Vec<String> {
    // Returns all columns inside of a table. Columns stay base64-encoded.
    let mut stmt = conn.prepare(&format!("PRAGMA table_info(\"{}\")", table_name)).expect("Invalid table.");
    let column_names: Vec<String> = stmt.query_map([], |row| row.get(1)).expect("Failed to get column names.")
        .collect::<Result<Vec<String>, _>>()
        .expect("Failed to collect results.");

    let filtered_column_names: Vec<String> = column_names.into_iter().filter(|column| column.to_string() != "description")
        .collect();
    filtered_column_names
}

pub fn get_all_columns_total(conn: &Connection) -> Vec<String> {
    //Helper function; returns all columns across all tables.
    let all_tables = get_all_tables(conn);

    let all_columns: Vec<String> = all_tables
        .iter()
        .flat_map(|table_name| get_columns_from_table(conn, &encode_base64(table_name)))
        .collect();

    all_columns
}

pub fn filter_for_description(conn: &Connection, input: &str) -> Vec<String> { // %<Word>% is a before-and-after wildcard in SQL.
    // Returns all descriptions from all tables that match a given filter. Can handle empty input and can also return an empty Vec.
    let mut return_vec: Vec<String> = vec![];

    let all_tables: Vec<String> = get_all_tables(conn);
    let mut all_descriptions: Vec<String> = vec![];

    for table in &all_tables {
        let mut stmt = conn.prepare(&format!("SELECT description FROM \"{}\"", encode_base64(table))).expect("");
        let descriptions_from_table = stmt.query_map([], |row| row.get(0)).expect("").collect::<Result<Vec<String>>>().expect("");
        for description in descriptions_from_table {
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

pub fn get_all_templates(conn: &Connection) -> Vec<Vec<u8>> {
    // Returns the structures of all templates as a Vec<Vec<u8>>. Can be decoded into JSON strings.
    let mut stmt = conn.prepare("SELECT structure FROM templates").expect("");

    let templates_structures: Vec<Vec<u8>> = stmt.query_map([], |row| row.get(0)).expect("Failed to get column names.")
        .collect::<Result<Vec<Vec<u8>>>>()
        .expect("Failed to collect results.");
    templates_structures
}

pub fn select_line(conn: &Connection, description: String, key: Vec<u8>) -> (String, Vec<(String, String)>) {
    // Dangerous: Returns table where description is found and also all columns with their corresponding DECRYPTED values. Might have to change.
    let encoded_table: String = conn.query_row(&format!("SELECT template FROM descriptions WHERE description = '{}'", encode_base64(&description)), params![], |row| row.get(0)).expect("");
    //let mut stmt = conn.prepare(&format!("SELECT * FROM \"{}\" WHERE description = '{}'", encoded_table, encode_base64(description))).expect("");
    //let args: Vec<String> = stmt.query_map([], |row| row.get(0)).expect("").collect::<Result<Vec<String>>>().expect("");
    let cols: Vec<String> = get_columns_from_table(conn, &encoded_table);
    let mut combined_vec: Vec<(String, String)> = vec![];

    for col in cols.iter() {
        combined_vec.push((decode_base64(col), select_entry(conn, decode_base64(&encoded_table), description.clone(), col.to_string(), key.clone())))
    };

    (decode_base64(encoded_table), combined_vec)
}

pub fn select_line_encrypted(conn: &Connection, description: String) -> (String, Vec<(String, Vec<u8>)>) {
    //select_line() but doesn't decrypt. Use this combined with decrypt_single_entry() instead of select_line.
    let encoded_table: String = conn.query_row(&format!("SELECT template FROM descriptions WHERE description = '{}'", encode_base64(&description)), params![], |row| row.get(0)).expect("");
    let cols: Vec<String> = get_columns_from_table(conn, &encoded_table);
    let mut combined_vec: Vec<(String, Vec<u8>)> = vec![];
    for col in cols.iter().skip(1) {
        let decrypted_val: Vec<u8> = conn.query_row(&format!("SELECT {} FROM {} WHERE description = '{}'", col, encoded_table, encode_base64(&description)), params![], |row| row.get(0)).expect("");
        combined_vec.push((decode_base64(col), decrypted_val));
    }
    
    (decode_base64(encoded_table), combined_vec)
}

pub fn decrypt_single_entry(conn: &Connection, description: String, column: String, key: Vec<u8>) -> String {
    //Decrypts just one entry instead of a whole row. Use this combined with select_line_encrypted() instead of select_line.
    let table_name: String = conn.query_row(&format!("SELECT template FROM descriptions WHERE description = '{}'", encode_base64(&description)), params![], |row| row.get(0)).expect("");
    let query_result: String = conn.query_row(&format!("SELECT \"{}\" FROM \"{}\" WHERE description = '{}'", encode_base64(&column), encode_base64(&table_name), encode_base64(&description)), params![], |row| row.get(0)).expect("");
    let stmt: String = conn.query_row(&format!("SELECT nonce FROM nonces WHERE orig_table = '{}' AND orig_entry = '{}' AND orig_desc = '{}'", encode_base64(&table_name), encode_base64(&column), encode_base64(&description)), params![], |row| row.get(0)).expect("");
    let nonce: Vec<u8> = base64::decode(stmt).expect("");

    let key_usable: GenericArray<u8, U32> = u32_from_slice(&key);
    let nonce_usable: GenericArray<u8, U12> = u12_from_slice(&nonce);

    decrypt_aesgcm(&key_usable, &nonce_usable, &base64::decode(query_result).expect(""))
}

// IMPLEMENTING SQL COMMANDS

pub fn create_table(conn: &Connection, table_name: String, columns: Vec<String>) -> Result<()> {
    //Create new template with columns
    conn.execute(&format!("CREATE TABLE \"{}\" (description TEXT, {})", encode_base64(table_name), columns.iter()
        .map(|column| format!("\"{}\" TEXT", encode_base64(column)))
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
    //args_str[0] is description!!!! = shown name of entry like Email, Skype, etc.!!!
    let description = args_str[0].clone().into_bytes();
    let mut enc_args_vec: Vec<Vec<u8>> = vec![];
    let key_as_array = u32_from_slice(&key);

    enc_args_vec.push(description.clone());

    let table_columns: Vec<String> = get_columns_from_table(&conn, &encode_base64(&table_name));

    for (col_index, arg) in args_str.iter().skip(1).enumerate() {
        loop {
            let nonce = nonce_generator(); //Generate nonces on the fly for every entry -> No nonce reuse attack
            if conn.query_row(&format!("SELECT 1 FROM nonces WHERE nonce = '{}'", encode_base64(&nonce)), params![], |_| Ok(1)).is_err() {
                //This query ensures that the generates nonce is unique; the odds of generating two same random 96 bit numbers are low, but never zero!
                let current_col: &str = &table_columns[col_index];
                conn.execute(&format!("INSERT INTO nonces VALUES('{}', '{}', '{}', '{}')", encode_base64(&nonce), encode_base64(&table_name), encode_base64(&description), current_col), params![]).expect("Something went wrong.");
                let enc_arg: Vec<u8> = encrypt_aesgcm(&key_as_array, &nonce, &arg);
                enc_args_vec.push(enc_arg);
                break;
            }
        }
    }
    let args_aes_b64: Vec<String> = enc_args_vec.iter().map(|ciphertext| encode_base64(ciphertext)).collect();

    let args_aes_b64_string: String = format_args(args_aes_b64); //add ' ', around all entries

    conn.execute(&format!("INSERT INTO \"{}\" VALUES({})", encode_base64(&table_name), args_aes_b64_string), params![])?;

    conn.execute(&format!("INSERT INTO descriptions VALUES('{}', '{}')", encode_base64(&args_str[0]), encode_base64(&table_name)), params![])?;

    Ok(())
}

pub fn select_entry(conn: &Connection, table_name: String, description: String, column: String, key: Vec<u8>) -> String {
    //Inverse order: Decode from Base64 -> Decrypt using AES and given nonce -> return lé value
    let query_result: String = conn.query_row(&format!("SELECT \"{}\" FROM \"{}\" WHERE description = '{}'", encode_base64(&column), encode_base64(&table_name), encode_base64(&description)), params![], |row| row.get(0)).expect("");
    let stmt: String = conn.query_row(&format!("SELECT nonce FROM nonces WHERE orig_table = '{}' AND orig_entry = '{}' AND orig_desc = '{}'", encode_base64(&table_name), &column, encode_base64(&description)), params![], |row| row.get(0)).expect("");
    let nonce: Vec<u8> = base64::decode(stmt).expect("");

    let key_usable: GenericArray<u8, U32> = u32_from_slice(&key);
    let nonce_usable: GenericArray<u8, U12> = u12_from_slice(&nonce);

    decrypt_aesgcm(&key_usable, &nonce_usable, &base64::decode(query_result).expect(""))
}

pub fn delete_entry(conn: &Connection, description: String) {
    //Deletes an entry and all associated information from other tables.
    let enc_table: String = conn.query_row(&format!("SELECT template FROM descriptions WHERE description = '{}'", encode_base64(&description)), params![], |row| row.get(0)).expect("");
    conn.execute(&format!("DELETE FROM \"{}\" WHERE description = '{}'", enc_table, encode_base64(&description)), params![]).expect("");
    conn.execute(&format!("DELETE FROM nonces WHERE orig_table = '{}' AND orig_desc = '{}'", enc_table, encode_base64(&description)), params![]).expect("");
    conn.execute(&format!("DELETE FROM descriptions WHERE description = '{}'", encode_base64(&description)), params![]).expect("");
}

pub fn update_entry(conn: &Connection, table_name: String, description: String, edited_entry: String, edited_column: String, key: Vec<u8>) -> Result<()> {
    //Not yet used nor tested!
    let key_usable: GenericArray<u8, U32> = u32_from_slice(&key);
    let nonce_usable: GenericArray<u8, U12> = loop {
        let nonce = nonce_generator(); //Generate nonces on the fly for every entry -> No nonce reuse attack
        if conn.query_row(&format!("SELECT 1 FROM nonces WHERE nonce = '{}'", encode_base64(&nonce)), params![], |_| Ok(1)).is_err() {
            //This query ensures that the generates nonce is unique; the odds of generating two same random 96 bit numbers are low, but never zero!
            break (nonce);
        }
    };

    let enc_message: Vec<u8> = encrypt_aesgcm(&key_usable, &nonce_usable, &edited_entry);

    conn.execute(&format!("UPDATE \"{}\" SET \"{}\" = '{}' WHERE description = '{}'", encode_base64(&table_name), encode_base64(&edited_column), encode_base64(&enc_message), encode_base64(&description)), params![])?;
    //If the database crashes between these queries, the database is going to be corrupted lol
    conn.execute(&format!("DELETE FROM nonces WHERE orig_table = '{}' AND orig_desc = '{}'", encode_base64(&table_name), encode_base64(&description)), params![]).expect("How did this not work, what");
    conn.execute(&format!("INSERT INTO nonces VALUES('{}', '{}', '{}', '{}')", encode_base64(&nonce_usable), encode_base64(&table_name), encode_base64(&description), encode_base64(&edited_column)), params![]).expect("Something went wrong.");

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

pub fn check_name_available(conn: &Connection, description: String) -> bool {
    conn.execute(&format!("SELECT 1 FROM descriptions WHERE description = '{}'", encode_base64(description)), params![]).is_ok()
}


pub fn decode_vec_string_b64(encoded_vec: Vec<String>) -> Vec<String> {
    //Decode a full vector of base64-encoded values
    let decoded_vec: Vec<String> = encoded_vec
        .iter()
        .map(|encoded_column| decode_base64(encoded_column.to_string()))
        .collect();
    decoded_vec
}