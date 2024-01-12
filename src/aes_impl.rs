use rand::{Rng, thread_rng};
use aes_gcm::{aead::{Aead, generic_array::GenericArray, KeyInit}, Aes256Gcm};
use typenum::{U12, U32, B1, B0, UInt, UTerm};

pub fn nonce_from_slice(slice: &[u8]) -> GenericArray<u8, U12> {
    let mut default_array: GenericArray<u8, U12> = GenericArray::default();
    default_array.clone_from_slice(&slice);
    default_array
}

pub fn array_from_slice(slice: &[u8]) -> GenericArray<u8, U32> {
    let mut array: GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>> = GenericArray::default();
    array.copy_from_slice(slice);
    array
}

pub fn nonce_generator() -> GenericArray<u8, U12> { //12 bytes = 96 bits: optimal nonce size for AES in GCM
    let mut rng = thread_rng();
    let random_num: i128 = rng.gen();
    let mut nonce_bytes = GenericArray::default();
    nonce_bytes.copy_from_slice(&random_num.to_be_bytes()[0..12]);
    nonce_bytes
}

pub fn encrypt_aesgcm(key: &GenericArray<u8, U32>, nonce: &GenericArray<u8, U12>, message: &str) -> Vec<u8> {
    let cipher = Aes256Gcm::new(&key.clone());
    let ciphertext = cipher.encrypt(nonce, message.as_bytes())
                                    .expect("Encryption failed");
    ciphertext
}

pub fn decrypt_aesgcm(key: &GenericArray<u8, U32>, nonce: &GenericArray<u8, U12>, ciphermessage: &Vec<u8>) -> String {
    let cipher = Aes256Gcm::new(&key.clone());
    let decrypted_text = cipher.decrypt(nonce, ciphermessage.as_slice());
    match decrypted_text {
        Ok(decrypted) => String::from_utf8(decrypted).expect("Conversion to String failed"),
        Err(_) => String::from("Decryption failed."),
    }
}