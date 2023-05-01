use rand::{thread_rng, RngCore, Error};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

use crate::persistence::Persistence;



pub fn get_random_key32() ->  Result<String, Error> {
    let mut arr = [0u8; 32];
    thread_rng().try_fill_bytes(&mut arr[..])?;
    Ok(hex::encode(arr))
}


pub  fn default_encrypt(to_encrypt: &str, persistence: &Persistence) -> String {
    match futures::executor::block_on(persistence.get("encryption", "default")) {
        Ok(key) => {
            let mc = new_magic_crypt!(key.value, 256);

            mc.encrypt_str_to_base64(to_encrypt)
        },
        Err(_err) => {
            "".to_string()
        }
    }    
}

pub  fn default_decrypt(to_decrypt: &str, persistence: &Persistence) -> String { 
    match futures::executor::block_on(persistence.get("encryption", "default")) {
        Ok(key) => {
            let mc = new_magic_crypt!(key.value, 256);

            mc.decrypt_base64_to_string(to_decrypt).unwrap()
        },
        Err(_err) => {
            "".to_string()
        }
    }    
}




pub fn encrypt(to_encrypt: String, key: &str) -> String {
    let mc = new_magic_crypt!(key, 256);

    mc.encrypt_str_to_base64(to_encrypt)
}

pub fn decrypt(to_decrypt: String, key: &str) -> String {
    let mc = new_magic_crypt!(key, 256);

    mc.decrypt_base64_to_string(to_decrypt).expect("Could not decrypt value. Did you change the crypt key?")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_key32() {
        assert_eq!(get_random_key32().unwrap().len(), 64);
    }

    #[test]
    fn test_roundtrip() {
        let key = "this is a key".to_string();
        
        let encrypted = encrypt("this is a text that should be encrypted and decrypted".to_string(), &key);

        println!("encrypted value: {}", encrypted);

        assert_ne!(encrypted, "this is a text that should be encrypted and decrypted");
        
        let decrypted = decrypt(encrypted, &key);

        assert_eq!(&decrypted, "this is a text that should be encrypted and decrypted");

        println!("decrypted value: {}", decrypted);
    }
}