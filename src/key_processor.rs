use shielded::Shielded;
use argon2::Argon2;


static SALT: &[u8; 14] = b"cru5tw0rd5a1ty";


// insert clear text key and return a vault
pub fn process_input_key(password: String) -> SecureStorage {
    let expand = derive_key(password);
    SecureStorage::new(expand)
}

fn derive_key(password: String) -> Vec<u8> {
    let mut key = [0u8; 64];

    Argon2::default().hash_password_into(
        password.as_ref(),
        SALT,
        &mut key,
    ).unwrap();

    key.to_vec()
}

struct SecureStorage {
    memory: Shielded,
}

impl SecureStorage {
    //  key gets deleted after being stored
    fn new(key_vector: Vec<u8>) -> SecureStorage {
        SecureStorage {
            memory: Shielded::new(key_vector),
        }
    }

    // Shielded re-encrypts key after being unshielded
    fn get_key(&mut self) -> Vec<u8> {
        let unshielded = self.memory.unshield();
        unshielded.as_ref().to_vec()
    }
}
