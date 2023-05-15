use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand::{thread_rng, Error, RngCore};

pub fn get_random_key32() -> Result<String, Error> {
    let mut arr = [0u8; 32];
    thread_rng().try_fill_bytes(&mut arr[..])?;
    Ok(hex::encode(arr))
}

pub fn default_encrypt(to_encrypt: &str, crypto_key: &str) -> String {
    let mc = new_magic_crypt!(crypto_key, 256);
    mc.encrypt_str_to_base64(to_encrypt)
}

pub fn default_decrypt(to_decrypt: &str, crypto_key: &str) -> String {
    let mc = new_magic_crypt!(crypto_key, 256);

    mc.decrypt_base64_to_string(to_decrypt).unwrap()
}

#[allow(dead_code)]
pub fn encrypt(to_encrypt: String, key: &str) -> String {
    let mc = new_magic_crypt!(key, 256);

    mc.encrypt_str_to_base64(to_encrypt)
}

#[allow(dead_code)]
pub fn decrypt(to_decrypt: String, key: &str) -> String {
    let mc = new_magic_crypt!(key, 256);

    mc.decrypt_base64_to_string(to_decrypt)
        .expect("Could not decrypt value. Did you change the crypt key?")
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

        let encrypted = encrypt(
            "this is a text that should be encrypted and decrypted".to_string(),
            &key,
        );

        println!("encrypted value: {}", encrypted);

        assert_ne!(
            encrypted,
            "this is a text that should be encrypted and decrypted"
        );

        let decrypted = decrypt(encrypted, &key);

        assert_eq!(
            &decrypted,
            "this is a text that should be encrypted and decrypted"
        );

        println!("decrypted value: {}", decrypted);
    }
}
