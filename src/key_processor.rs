use argon2::{Argon2, Algorithm, Version, Params};
use shielded::Shielded;
use std::thread::available_parallelism;


static SALT: &[u8; 14] = b"cru5tw0rd5a1ty";


// insert clear text key and return a vault
pub fn process_input_key(password: String) -> SecureStorage {
    let expand = derive_key(password);
    SecureStorage::new(expand)
}

fn derive_key(password: String) -> Vec<u8> {
    let mut key = [0u8; 64];

    // uses half of the available logical cpu cores
    let config = Argon2::new(
        Algorithm::Argon2d,
        Version::default(),
        Params::new(
            Params::DEFAULT_M_COST,
            4,
            available_parallelism()
                .unwrap_or(NonZeroUsize::new(1).unwrap())
                .get()
                as u32
                / 2
                .max(1),
            Some(key.len()),
        ).unwrap(),
    );

    config.hash_password_into(
        password.as_bytes(),
        SALT,
        &mut key,
    ).unwrap();

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

    // Shielded re-encrypts key after being unshielded
    pub fn get_key(&mut self) -> Vec<u8> {
        let unshielded = self.memory.unshield();
        unshielded.as_ref().to_vec()
    }
}
