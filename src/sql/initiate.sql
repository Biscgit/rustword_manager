-- create templates table
CREATE TABLE IF NOT EXISTS templates
(
    template_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT,
    structure   BLOB
);

-- username-password template
CREATE TABLE IF NOT EXISTS tp_simple
(
    description TEXT UNIQUE,
    clear_1     TEXT,
    hidden_1    Text
);
INSERT INTO templates (name, structure)
VALUES ('Simple', CAST('{
  "clear_1": "username",
  "hidden_1": "password"
}' AS BLOB));

-- ssh key pair
CREATE TABLE IF NOT EXISTS tp_ssh_keypair
(
    description TEXT UNIQUE,
    clear_1     TEXT,
    clear_2     Text,
    hidden_1    Text
);
INSERT INTO templates (name, structure)
VALUES ('SSH-Keypair', CAST('{
  "clear_1": "name",
  "clear_2": "public_key",
  "hidden_1": "private_key"
}' AS BLOB));

CREATE TABLE IF NOT EXISTS nonces
(
  nonce TEXT UNIQUE,
  orig_table TEXT,
  orig_desc TEXT,
  orig_entry TEXT
);
