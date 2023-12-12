use shielded::Shielded;


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
