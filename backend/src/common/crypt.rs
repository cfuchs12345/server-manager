use aes::cipher::consts::U16;
use aes_gcm::{
    aead::{Aead, KeyInit},
    aes::Aes256, // Or `Aes128Gcm`
    AesGcm,
    Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use bcrypt::{hash, verify, DEFAULT_COST};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use pbkdf2::pbkdf2_hmac_array;
use rand::{thread_rng, RngCore};
use sha2::Sha256;

use crate::models::error::AppError;

pub type Aes256Gcm16 = AesGcm<Aes256, U16>;

pub fn get_random_key32() -> Result<String, AppError> {
    let mut arr = [0u8; 32];
    thread_rng().try_fill_bytes(&mut arr[..]).map_err(|err| {
        AppError::Unknown(format!(
            "Could not generate random string. Error was  {}",
            err
        ))
    })?;
    Ok(hex::encode(arr))
}

pub fn default_encrypt(to_encrypt: &str, crypto_key: &str) -> Result<String, AppError> {
    let mc = new_magic_crypt!(crypto_key, 256);
    Ok(mc.encrypt_str_to_base64(to_encrypt))
}

pub fn default_decrypt(to_decrypt: &str, crypto_key: &str) -> Result<String, AppError> {
    let mc = new_magic_crypt!(crypto_key, 256);

    mc.decrypt_base64_to_string(to_decrypt)
        .map_err(AppError::from)
}

pub fn aes_decrypt(to_decrypt: &str, secret: &str) -> Result<String, AppError> {
    let bytes = general_purpose::STANDARD
        .decode(to_decrypt)
        .map_err(|e| AppError::Unknown(format!("{}", e)))?;

    let salt = &bytes[..64];
    let iv = &bytes[64..64 + 16];
    let text = &bytes[64 + 16..]; // including tag postfix

    let key = pbkdf2_hmac_array::<Sha256, 32>(secret.as_bytes(), salt, 100000);

    let cipher =
        Aes256Gcm16::new_from_slice(&key).map_err(|e| AppError::Unknown(format!("{}", e)))?;

    // nonce / iv from sender
    let nonce = Nonce::from_slice(iv);
    match cipher.decrypt(nonce, text) {
        Ok(decrypted) => {
            Ok(String::from_utf8(decrypted).map_err(|e| AppError::Unknown(format!("{}", e)))?)
        }
        Err(_err) => Err(AppError::DecryptionError),
    }
}

pub fn make_aes_secrect(user_id: &str, otk: &str) -> String {
    let fp = if user_id.len() > 5 {
        &user_id[user_id.len() - 5..]
    } else {
        user_id
    };
    let sp = &otk[..otk.len() - fp.len()];
    format!("{}{}", fp, sp)
}

pub fn hash_password(to_hash: &str) -> Result<String, AppError> {
    hash(to_hash, DEFAULT_COST).map_err(AppError::from)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, AppError> {
    verify(password, hashed_password).map_err(AppError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_key32() {
        assert_eq!(get_random_key32().expect("should not happen").len(), 64);
    }

    #[test]
    fn test_roundtrip() {
        let key = "this is a key";
        let input = "this is a text that should be encrypted and decrypted";

        let encrypted = default_encrypt(input, key).expect("should not happen");
        assert_ne!(encrypted, input);

        let decrypted = default_decrypt(encrypted.as_str(), key).expect("should not happen");
        assert_eq!(&decrypted, input);
    }
}
