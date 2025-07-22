use chacha20poly1305::{
    aead::{Aead, NewAead, Payload},
    ChaCha20Poly1305, Key, Nonce,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AeadError {
    #[error("Encryption error")]
    EncryptionError,
    #[error("Decryption error")]
    DecryptionError,
    #[error("Invalid key size")]
    InvalidKeySize,
}

pub trait AeadCipher: Send + Sync {
    fn encrypt(&self, nonce: &[u8], data: &[u8], aad: &[u8]) -> Result<Vec<u8>, AeadError>;
    fn decrypt(&self, nonce: &[u8], data: &[u8], aad: &[u8]) -> Result<Vec<u8>, AeadError>;
}

pub struct ChaChaPolyAead {
    cipher: ChaCha20Poly1305,
}

impl ChaChaPolyAead {
    pub fn new(key: &[u8]) -> Result<Self, AeadError> {
        if key.len() != 32 {
            return Err(AeadError::InvalidKeySize);
        }

        let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
        Ok(Self { cipher })
    }
}

impl AeadCipher for ChaChaPolyAead {
    fn encrypt(&self, nonce: &[u8], data: &[u8], aad: &[u8]) -> Result<Vec<u8>, AeadError> {
        let nonce = Nonce::from_slice(nonce);
        let payload = Payload { msg: data, aad };
        
        self.cipher
            .encrypt(nonce, payload)
            .map_err(|_| AeadError::EncryptionError)
    }

    fn decrypt(&self, nonce: &[u8], data: &[u8], aad: &[u8]) -> Result<Vec<u8>, AeadError> {
        let nonce = Nonce::from_slice(nonce);
        let payload = Payload { msg: data, aad };
        
        self.cipher
            .decrypt(nonce, payload)
            .map_err(|_| AeadError::DecryptionError)
    }
}
