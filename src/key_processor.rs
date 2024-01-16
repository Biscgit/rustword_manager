use argon2::{Algorithm, Argon2, Params, Version};
use shielded::Shielded;
use std::num::NonZeroUsize;
use std::thread::available_parallelism;

static SALT: &[u8; 16] = b"cru5tw0rd5a1ty!!";

fn derive_key(password: String) -> Vec<u8> {
    let mut key = [0u8; 64];

    // uses half of the available logical cpu cores
    // https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
    let config = Argon2::new(
        Algorithm::Argon2d,
        Version::default(),
        Params::new(
            1024 * 256,
            10,
            available_parallelism()
                .unwrap_or(NonZeroUsize::new(1).unwrap())
                .get() as u32
                / 2.max(1),
            Some(key.len()),
        )
        .unwrap(),
    );

    config
        .hash_password_into(password.as_bytes(), SALT, &mut key)
        .unwrap();

    key.to_vec()
}

pub struct SecureStorage {
    memory: Shielded,
}

impl SecureStorage {
    //  key gets deleted after being stored
    fn new(key_vector: Vec<u8>) -> SecureStorage {
        SecureStorage {
            memory: Shielded::new(key_vector),
        }
    }

    pub fn from_string(input: String) -> SecureStorage {
        // create new shielded memory from a string
        let buffer = input.as_bytes().to_vec();
        SecureStorage {
            memory: Shielded::new(buffer),
        }
    }

    pub fn get_contents(&mut self) -> Vec<u8> {
        // shielded re-encrypts key after being unshielded

        let unshielded = self.memory.unshield();
        unshielded.as_ref().to_vec()
    }
}
