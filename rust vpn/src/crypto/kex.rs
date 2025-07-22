use std::sync::Arc;
use thiserror::Error;
use x25519_dalek::{PublicKey, StaticSecret};
use rand::rngs::OsRng;

use super::{CryptoProvider, CryptoError};

#[derive(Debug, Error)]
pub enum KexError {
    #[error("Crypto provider error: {0}")]
    CryptoError(#[from] CryptoError),
    #[error("Invalid key exchange mode")]
    InvalidMode,
}

#[derive(Debug, Clone, Copy)]
pub enum KemMode {
    PqcOnly,
    Hybrid,
    Classical,
}

pub struct KeyExchange<P: CryptoProvider> {
    mode: KemMode,
    crypto_provider: Arc<P>,
}

impl<P: CryptoProvider> KeyExchange<P> {
    pub fn new(crypto_provider: Arc<P>, mode: KemMode) -> Self {
        Self {
            mode,
            crypto_provider,
        }
    }

    pub fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), KexError> {
        match self.mode {
            KemMode::PqcOnly => {
                self.crypto_provider.generate_static_keypair()
                    .map_err(KexError::from)
            }
            KemMode::Classical => {
                let secret = StaticSecret::random_from_rng(OsRng);
                let public = PublicKey::from(&secret);
                Ok((secret.to_bytes().to_vec(), public.to_bytes().to_vec()))
            }
            KemMode::Hybrid => {
                // Generate both classical and PQC keypairs
                let (pqc_pk, pqc_sk) = self.crypto_provider.generate_static_keypair()?;
                let secret = StaticSecret::random_from_rng(OsRng);
                let public = PublicKey::from(&secret);
                
                // Combine keys
                let mut combined_pk = Vec::with_capacity(pqc_pk.len() + 32);
                combined_pk.extend_from_slice(&pqc_pk);
                combined_pk.extend_from_slice(&public.to_bytes());
                
                let mut combined_sk = Vec::with_capacity(pqc_sk.len() + 32);
                combined_sk.extend_from_slice(&pqc_sk);
                combined_sk.extend_from_slice(&secret.to_bytes());
                
                Ok((combined_sk, combined_pk))
            }
        }
    }

    pub fn encapsulate(&self, peer_pk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), KexError> {
        match self.mode {
            KemMode::PqcOnly => {
                self.crypto_provider.encapsulate_static(peer_pk)
                    .map_err(KexError::from)
            }
            KemMode::Classical => {
                let peer_key = PublicKey::from(<[u8; 32]>::try_from(peer_pk).map_err(|_| KexError::InvalidMode)?);
                let secret = StaticSecret::random_from_rng(OsRng);
                let public = PublicKey::from(&secret);
                let shared = secret.diffie_hellman(&peer_key);
                
                Ok((shared.to_bytes().to_vec(), public.to_bytes().to_vec()))
            }
            KemMode::Hybrid => {
                // Split combined public key
                let (pqc_pk, classical_pk) = peer_pk.split_at(peer_pk.len() - 32);
                
                // Perform PQC KEM
                let (pqc_ss, pqc_ct) = self.crypto_provider.encapsulate_static(pqc_pk)?;
                
                // Perform classical ECDH
                let peer_key = PublicKey::from(<[u8; 32]>::try_from(classical_pk).map_err(|_| KexError::InvalidMode)?);
                let secret = StaticSecret::random_from_rng(OsRng);
                let public = PublicKey::from(&secret);
                let classical_ss = secret.diffie_hellman(&peer_key);
                
                // Combine shared secrets and ciphertexts
                let mut combined_ss = Vec::with_capacity(pqc_ss.len() + 32);
                combined_ss.extend_from_slice(&pqc_ss);
                combined_ss.extend_from_slice(&classical_ss.to_bytes());
                
                let mut combined_ct = Vec::with_capacity(pqc_ct.len() + 32);
                combined_ct.extend_from_slice(&pqc_ct);
                combined_ct.extend_from_slice(&public.to_bytes());
                
                Ok((combined_ss, combined_ct))
            }
        }
    }

    pub fn decapsulate(&self, ciphertext: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, KexError> {
        match self.mode {
            KemMode::PqcOnly => {
                self.crypto_provider.decapsulate_static(ciphertext, secret_key)
                    .map_err(KexError::from)
            }
            KemMode::Classical => {
                let secret = StaticSecret::from(<[u8; 32]>::try_from(secret_key).map_err(|_| KexError::InvalidMode)?);
                let peer_key = PublicKey::from(<[u8; 32]>::try_from(ciphertext).map_err(|_| KexError::InvalidMode)?);
                let shared = secret.diffie_hellman(&peer_key);
                Ok(shared.to_bytes().to_vec())
            }
            KemMode::Hybrid => {
                // Split combined keys
                let (pqc_sk, classical_sk) = secret_key.split_at(secret_key.len() - 32);
                let (pqc_ct, classical_ct) = ciphertext.split_at(ciphertext.len() - 32);
                
                // Perform PQC KEM decapsulation
                let pqc_ss = self.crypto_provider.decapsulate_static(pqc_ct, pqc_sk)?;
                
                // Perform classical ECDH
                let secret = StaticSecret::from(<[u8; 32]>::try_from(classical_sk).map_err(|_| KexError::InvalidMode)?);
                let peer_key = PublicKey::from(<[u8; 32]>::try_from(classical_ct).map_err(|_| KexError::InvalidMode)?);
                let classical_ss = secret.diffie_hellman(&peer_key);
                
                // Combine shared secrets
                let mut combined_ss = Vec::with_capacity(pqc_ss.len() + 32);
                combined_ss.extend_from_slice(&pqc_ss);
                combined_ss.extend_from_slice(&classical_ss.to_bytes());
                
                Ok(combined_ss)
            }
        }
    }
}
