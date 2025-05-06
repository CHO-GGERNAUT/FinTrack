use aes_gcm::{
    AeadCore, Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit, OsRng},
};

use crate::application::interfaces::services::encryption_service::{
    EncryptionError, EncryptionService,
};

const NONCE_SIZE: usize = 12;

#[derive(Clone)]
pub struct AesGcmEncryptionService {
    key: Key<Aes256Gcm>,
}

impl AesGcmEncryptionService {
    pub fn new(key_hex: &str) -> Result<Self, EncryptionError> {
        let key_bytes = hex::decode(key_hex)
            .map_err(|e| EncryptionError::EncryptionFailed(format!("Invalid hex key: {}", e)))?;

        if key_bytes.len() != 32 {
            return Err(EncryptionError::InvalidKeyLength);
        }

        let key = Key::<Aes256Gcm>::from_slice(&key_bytes).clone();
        Ok(Self { key })
    }
}

impl EncryptionService for AesGcmEncryptionService {
    fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>, EncryptionError> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

        let mut result = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
        result.extend_from_slice(nonce.as_slice());
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    fn decrypt(&self, ciphertext_with_nonce: &[u8]) -> Result<String, EncryptionError> {
        if ciphertext_with_nonce.len() < NONCE_SIZE {
            return Err(EncryptionError::FormatError(
                "Ciphertext too short to contain nonce".to_string(),
            ));
        }

        let (nonce_bytes, ciphertext) = ciphertext_with_nonce.split_at(NONCE_SIZE);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = Aes256Gcm::new(&self.key);

        let decrypted_bytes = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;

        String::from_utf8(decrypted_bytes).map_err(|e| {
            EncryptionError::FormatError(format!("Decrypted data is not valid UTF-8: {}", e))
        })
    }
}
