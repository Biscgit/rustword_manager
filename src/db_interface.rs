use rusqlite::{Connection, params};

fn establish_connection(db_name: &str, db_key: &str) {
    let db_path = db_name;
    let key = db_key;

    let conn = Connection::open(db_path)
        .expect("Failed to open database");

    conn.execute_batch(&format!("PRAGMA key = '{}'", key))
        .expect("Failed to set encryption key");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS check_decryption (
                  check_var INTEGER PRIMARY KEY)",
        params![],
    )
    .expect("Failed to create table");

    //Should be 0; default query to check if decryption failed
    let count: i32 = conn
        .query_row("SELECT COUNT(*) FROM check_decryption", params![], |row| row.get(0))
        .expect("Failed to get count.");

    if count == 0 {
        println!("Decryption successful.")
    }
    else {
        println!("Decryption failed.");
    }
}
