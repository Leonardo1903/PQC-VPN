use oqs::kem::{self, Kem};
use oqs::sig::{self, Sig};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("KEM error: {0}")]
    KemError(String),
    #[error("Signature error: {0}")]
    SignatureError(String),
    #[error("AEAD error: {0}")]
    AeadError(String),
    #[error("Invalid key size")]
    InvalidKeySize,
}

pub trait CryptoProvider: Send + Sync + 'static {
    /// Initialize static KEM (McEliece)
    fn init_static_kem(&self) -> Result<Arc<Kem>, CryptoError>;

    /// Initialize ephemeral KEM (Kyber)
    fn init_ephemeral_kem(&self) -> Result<Arc<Kem>, CryptoError>;

    /// Initialize signature scheme (Dilithium)
    fn init_signature(&self) -> Result<Arc<Sig>, CryptoError>;

    /// Generate static keypair
    fn generate_static_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), CryptoError>;

    /// Generate ephemeral keypair
    fn generate_ephemeral_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), CryptoError>;

    /// Encapsulate key with static public key
    fn encapsulate_static(&self, pk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError>;

    /// Decapsulate key with static private key
    fn decapsulate_static(&self, ct: &[u8], sk: &[u8]) -> Result<Vec<u8>, CryptoError>;

    /// Encapsulate key with ephemeral public key
    fn encapsulate_ephemeral(&self, pk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError>;

    /// Decapsulate key with ephemeral private key
    fn decapsulate_ephemeral(&self, ct: &[u8], sk: &[u8]) -> Result<Vec<u8>, CryptoError>;

    /// Sign a message
    fn sign(&self, msg: &[u8], sk: &[u8]) -> Result<Vec<u8>, CryptoError>;

    /// Verify a signature
    fn verify(&self, msg: &[u8], sig: &[u8], pk: &[u8]) -> Result<bool, CryptoError>;
}

pub struct PqcWireguardCryptoProvider {
    static_kem: Arc<Kem>,
    ephemeral_kem: Arc<Kem>,
    signature: Arc<Sig>,
}

impl PqcWireguardCryptoProvider {
    pub fn new() -> Result<Self, CryptoError> {
        let static_kem = Arc::new(
            Kem::new(kem::Algorithm::Classic1_460896)
                .map_err(|e| CryptoError::KemError(e.to_string()))?,
        );

        let ephemeral_kem = Arc::new(
            Kem::new(kem::Algorithm::Kyber768).map_err(|e| CryptoError::KemError(e.to_string()))?,
        );

        let signature = Arc::new(
            Sig::new(sig::Algorithm::Dilithium2)
                .map_err(|e| CryptoError::SignatureError(e.to_string()))?,
        );

        Ok(Self {
            static_kem,
            ephemeral_kem,
            signature,
        })
    }
}

impl CryptoProvider for PqcWireguardCryptoProvider {
    fn init_static_kem(&self) -> Result<Arc<Kem>, CryptoError> {
        Ok(self.static_kem.clone())
    }

    fn init_ephemeral_kem(&self) -> Result<Arc<Kem>, CryptoError> {
        Ok(self.ephemeral_kem.clone())
    }

    fn init_signature(&self) -> Result<Arc<Sig>, CryptoError> {
        Ok(self.signature.clone())
    }

    fn generate_static_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
        self.static_kem
            .keypair()
            .map_err(|e| CryptoError::KemError(e.to_string()))
    }

    fn generate_ephemeral_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
        self.ephemeral_kem
            .keypair()
            .map_err(|e| CryptoError::KemError(e.to_string()))
    }

    fn encapsulate_static(&self, pk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
        self.static_kem
            .encapsulate(pk)
            .map_err(|e| CryptoError::KemError(e.to_string()))
    }

    fn decapsulate_static(&self, ct: &[u8], sk: &[u8]) -> Result<Vec<u8>, CryptoError> {
        self.static_kem
            .decapsulate(ct, sk)
            .map_err(|e| CryptoError::KemError(e.to_string()))
    }

    fn encapsulate_ephemeral(&self, pk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
        self.ephemeral_kem
            .encapsulate(pk)
            .map_err(|e| CryptoError::KemError(e.to_string()))
    }

    fn decapsulate_ephemeral(&self, ct: &[u8], sk: &[u8]) -> Result<Vec<u8>, CryptoError> {
        self.ephemeral_kem
            .decapsulate(ct, sk)
            .map_err(|e| CryptoError::KemError(e.to_string()))
    }

    fn sign(&self, msg: &[u8], sk: &[u8]) -> Result<Vec<u8>, CryptoError> {
        self.signature
            .sign(msg, sk)
            .map_err(|e| CryptoError::SignatureError(e.to_string()))
    }

    fn verify(&self, msg: &[u8], sig: &[u8], pk: &[u8]) -> Result<bool, CryptoError> {
        self.signature
            .verify(msg, sig, pk)
            .map_err(|e| CryptoError::SignatureError(e.to_string()))
    }
}
