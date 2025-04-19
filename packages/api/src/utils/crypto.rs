use anyhow::Context;
use ring::aead;

// Read encryption key from environment or config
fn get_encryption_key() -> anyhow::Result<aead::LessSafeKey> {
    // In production, this would be fetched from environment variables or a secure key store
    let key_data = std::env::var("ENCRYPTION_KEY")
        .unwrap_or_else(|_| "ENCRYPTION_KEY_SHOULD_BE_32_BYTES_LONG!".to_string())
        .into_bytes();

    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_data)
        .context("Failed to create encryption key")?;

    Ok(aead::LessSafeKey::new(unbound_key))
}

pub fn encrypt_sensitive_data(_data: &[u8]) -> anyhow::Result<Vec<u8>> {
    Ok(vec![])
}

pub fn decrypt_sensitive_data(encrypted_data: &[u8]) -> anyhow::Result<Vec<u8>> {
    if encrypted_data.len() < 12 {
        anyhow::bail!("Encrypted data too short");
    }

    let key = get_encryption_key()?;

    // Extract the nonce from the first 12 bytes
    let nonce_values: [u8; 12] = encrypted_data[..12]
        .try_into()
        .context("Failed to extract nonce")?;
    let nonce = aead::Nonce::assume_unique_for_key(nonce_values);

    // Decrypt the data (remaining bytes after nonce)
    let mut in_out = encrypted_data[12..].to_vec();
    let decrypted = key
        .open_in_place(nonce, aead::Aad::empty(), &mut in_out)
        .context("Failed to decrypt data")?;

    Ok(decrypted.to_vec())
}
