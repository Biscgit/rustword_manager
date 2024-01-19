-- create templates table
CREATE TABLE IF NOT EXISTS templates
(
    template_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT,
    structure   BLOB
);

-- username-password template
CREATE TABLE IF NOT EXISTS "dHBfd2ViX2NyZWRlbnRpYWw="
(
    description TEXT UNIQUE,
    "Y2xlYXJfMQ=="     TEXT,
    "aGlkZGVuXzE="    Text
);
INSERT INTO templates (name, structure)
VALUES ('Web Credential', CAST('{
  "deletable": false,
  "name": "Web Credential",
  "db_name": "tp_web_credential",
  "elements": [
    {
      "name": "Name",
      "private": false
    },
    {
      "name": "Username",
      "private": false
    },
    {
      "name": "Password",
      "private": true
    }
  ]
}' AS BLOB));

-- ssh key pair
CREATE TABLE IF NOT EXISTS "dHBfc3NoX2tleXBhaXI="
(
    description TEXT UNIQUE,
    "Y2xlYXJfMQ=="     TEXT,
    "Y2xlYXJfMg=="     Text,
    "aGlkZGVuXzE="    Text
);
INSERT INTO templates (name, structure)
VALUES ('SSH-Keypair', CAST('{
  "deletable": false,
  "name": "SSH-Keypair",
  "db_name": "tp_ssh_keypair",
  "elements": [
    {
      "name": "Name",
      "private": false
    },
    {
      "name": "Website",
      "private": false
    },
    {
      "name": "SSH-Public",
      "private": false
    },
    {
      "name": "SSH-Private",
      "private": true
    }
  ]
}' AS BLOB));

-- banking information
CREATE TABLE IF NOT EXISTS "dHBfc3NoX2tleXBhaXI="
(
    description TEXT UNIQUE,
    "Y2xlYXJfMQ=="     TEXT,
    "aGlkZGVuXzE="    Text,
    "aGlkZGVuXzI="    Text
);
INSERT INTO templates (name, structure)
VALUES ('Banking', CAST('{
  "deletable": false,
  "name": "Banking",
  "db_name": "tp_banking",
  "elements": [
    {
      "name": "Name",
      "private": false
    },
    {
      "name": "Bank",
      "private": false
    },
    {
      "name": "IBAN",
      "private": true
    },
    {
      "name": "Kontonummer",
      "private": true
    }
  ]
}' AS BLOB));

CREATE TABLE IF NOT EXISTS nonces
(
    nonce      TEXT UNIQUE,
    orig_table TEXT,
    orig_desc  TEXT,
    orig_entry TEXT
);

CREATE TABLE IF NOT EXISTS descriptions
(
    description TEXT UNIQUE,
    template    TEXT
);