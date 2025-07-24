use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use pqcrypto_dilithium::dilithium2::{
    detached_sign, keypair as dilithium_keypair, verify_detached_signature,
};
use pqcrypto_kyber::kyber768::{decapsulate, encapsulate, keypair as kyber_keypair};
use pqcrypto_traits::kem::{
    Ciphertext, PublicKey as KemPublicKey, SecretKey as KemSecretKey, SharedSecret,
};
use pqcrypto_traits::sign::{
    DetachedSignature, PublicKey as SignPublicKey, SecretKey as SignSecretKey,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Key exchange failed")]
    KeyExchangeError,
    #[error("Signature verification failed")]
    SignatureError,
    #[error("Encryption failed: {0}")]
    EncryptionError(String),
    #[error("Decryption failed: {0}")]
    DecryptionError(String),
}

pub struct CryptoSession {
    shared_key: Vec<u8>,
    cipher: Aes256Gcm,
    nonce_counter: u64,
}

impl CryptoSession {
    pub fn new(shared_key: Vec<u8>) -> Result<Self, CryptoError> {
        let key = Key::<Aes256Gcm>::from_slice(&shared_key);
        let cipher = Aes256Gcm::new(key);

        Ok(Self {
            shared_key,
            cipher,
            nonce_counter: 0,
        })
    }

    pub fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[4..12].copy_from_slice(&self.nonce_counter.to_be_bytes());
        self.nonce_counter = self.nonce_counter.wrapping_add(1);

        let nonce = Nonce::from_slice(&nonce_bytes);
        self.cipher
            .encrypt(nonce, data)
            .map_err(|e| CryptoError::EncryptionError(e.to_string()))
    }

    pub fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[4..12].copy_from_slice(&self.nonce_counter.to_be_bytes());
        self.nonce_counter = self.nonce_counter.wrapping_add(1);

        let nonce = Nonce::from_slice(&nonce_bytes);
        self.cipher
            .decrypt(nonce, data)
            .map_err(|e| CryptoError::DecryptionError(e.to_string()))
    }
}

pub struct KeyExchange {
    kyber_secret_key: Vec<u8>,
    kyber_public_key: Vec<u8>,
    dilithium_secret_key: Vec<u8>,
    dilithium_public_key: Vec<u8>,
}

impl KeyExchange {
    pub fn new() -> Self {
        let (kyber_public_key, kyber_secret_key) = kyber_keypair();
        let (dilithium_public_key, dilithium_secret_key) = dilithium_keypair();

        Self {
            kyber_secret_key: kyber_secret_key.as_bytes().to_vec(),
            kyber_public_key: kyber_public_key.as_bytes().to_vec(),
            dilithium_secret_key: dilithium_secret_key.as_bytes().to_vec(),
            dilithium_public_key: dilithium_public_key.as_bytes().to_vec(),
        }
    }

    pub fn get_public_keys(&self) -> (Vec<u8>, Vec<u8>) {
        (
            self.kyber_public_key.clone(),
            self.dilithium_public_key.clone(),
        )
    }

    pub fn verify_client_signature(
        &self,
        data: &[u8],
        signature: &[u8],
        client_public_key: &[u8],
    ) -> Result<(), CryptoError> {
        let sig = pqcrypto_dilithium::dilithium2::DetachedSignature::from_bytes(signature)
            .map_err(|_| CryptoError::SignatureError)?;
        let pk = pqcrypto_dilithium::dilithium2::PublicKey::from_bytes(client_public_key)
            .map_err(|_| CryptoError::SignatureError)?;

        verify_detached_signature(&sig, data, &pk).map_err(|_| CryptoError::SignatureError)
    }

    pub fn sign_data(&self, data: &[u8]) -> Vec<u8> {
        let sk = pqcrypto_dilithium::dilithium2::SecretKey::from_bytes(&self.dilithium_secret_key)
            .unwrap();
        detached_sign(data, &sk).as_bytes().to_vec()
    }

    pub fn process_client_key(&self, client_public_key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let pk = pqcrypto_kyber::kyber768::PublicKey::from_bytes(client_public_key)
            .map_err(|_| CryptoError::KeyExchangeError)?;
        let (_ciphertext, shared_secret) = encapsulate(&pk);
        Ok(shared_secret.as_bytes().to_vec())
    }

    pub fn process_server_response(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let ct = pqcrypto_kyber::kyber768::Ciphertext::from_bytes(ciphertext)
            .map_err(|_| CryptoError::KeyExchangeError)?;
        let sk = pqcrypto_kyber::kyber768::SecretKey::from_bytes(&self.kyber_secret_key)
            .map_err(|_| CryptoError::KeyExchangeError)?;
        let shared_secret = decapsulate(&ct, &sk);
        Ok(shared_secret.as_bytes().to_vec())
    }
}
