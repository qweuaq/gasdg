//! AES-256-GCM encrypted storage for sensitive configuration data.

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

const NONCE_LEN: usize = 12;

/// Encrypted file storage backed by AES-256-GCM.
pub struct EncryptedStorage {
    dir: PathBuf,
    key: [u8; 32],
}

impl EncryptedStorage {
    /// Create a new storage instance. The `passphrase` is hashed with SHA-256
    /// to derive the encryption key.
    pub fn new(dir: PathBuf, passphrase: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(passphrase.as_bytes());
        let key: [u8; 32] = hasher.finalize().into();
        Self { dir, key }
    }

    /// Write `data` encrypted to the given filename.
    pub fn write(&self, filename: &str, data: &[u8]) -> Result<(), String> {
        std::fs::create_dir_all(&self.dir).map_err(|e| e.to_string())?;

        let cipher = Aes256Gcm::new_from_slice(&self.key).map_err(|e| e.to_string())?;

        let mut nonce_bytes = [0u8; NONCE_LEN];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, data).map_err(|e| e.to_string())?;

        let mut out = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        out.extend_from_slice(&nonce_bytes);
        out.extend_from_slice(&ciphertext);

        std::fs::write(self.path(filename), out).map_err(|e| e.to_string())
    }

    /// Read and decrypt the given filename.
    pub fn read(&self, filename: &str) -> Result<Vec<u8>, String> {
        let raw = std::fs::read(self.path(filename)).map_err(|e| e.to_string())?;
        if raw.len() < NONCE_LEN {
            return Err("Encrypted file too short".into());
        }

        let (nonce_bytes, ciphertext) = raw.split_at(NONCE_LEN);
        let nonce = Nonce::from_slice(nonce_bytes);
        let cipher = Aes256Gcm::new_from_slice(&self.key).map_err(|e| e.to_string())?;

        cipher.decrypt(nonce, ciphertext).map_err(|e| e.to_string())
    }

    /// Store a JSON-serializable value.
    pub fn write_json<T: serde::Serialize>(&self, filename: &str, value: &T) -> Result<(), String> {
        let json = serde_json::to_vec(value).map_err(|e| e.to_string())?;
        self.write(filename, &json)
    }

    /// Read a JSON-deserializable value.
    pub fn read_json<T: serde::de::DeserializeOwned>(&self, filename: &str) -> Result<T, String> {
        let data = self.read(filename)?;
        serde_json::from_slice(&data).map_err(|e| e.to_string())
    }

    fn path(&self, filename: &str) -> PathBuf {
        self.dir.join(filename)
    }
}
