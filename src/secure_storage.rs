use std::str::from_utf8;
use shielded::Shielded;

// Shielded memory interface for data of String type like encryption-keys.
pub struct SecureStorage {
    memory: Shielded,
}

impl SecureStorage {
    // passing ownership -> key gets deleted after being stored
    pub fn new(key: String) -> SecureStorage {
        let buf: Vec<u8> = key.as_bytes().to_vec();

        SecureStorage {
            memory: Shielded::new(buf),
        }
    }

    // Shielded re-encrypts key after being unshielded
    pub fn get_key(&mut self) -> String {
        let unshielded = self.memory.unshield();
        let key = from_utf8(unshielded.as_ref()).unwrap();
        String::from(key)
    }
}
