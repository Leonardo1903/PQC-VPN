use std::sync::Arc;
use thiserror::Error;
use oqs::sig::{self, Sig};

#[derive(Debug, Error)]
pub enum SignatureError {
    #[error("Signing error: {0}")]
    SigningError(String),
    #[error("Verification error: {0}")]
    VerificationError(String),
    #[error("Invalid algorithm")]
    InvalidAlgorithm,
}

#[derive(Debug, Clone, Copy)]
pub enum SignatureScheme {
    Dilithium2,
    Falcon512,
    SphincsPlus,
}

impl SignatureScheme {
    fn to_algorithm(&self) -> sig::Algorithm {
        match self {
            SignatureScheme::Dilithium2 => sig::Algorithm::Dilithium2,
            SignatureScheme::Falcon512 => sig::Algorithm::Falcon512,
            SignatureScheme::SphincsPlus => sig::Algorithm::SphincsShake128sSimple,
        }
    }
}

pub struct SignatureProvider {
    signer: Arc<Sig>,
}

impl SignatureProvider {
    pub fn new(scheme: SignatureScheme) -> Result<Self, SignatureError> {
        let signer = Arc::new(Sig::new(scheme.to_algorithm())
            .map_err(|e| SignatureError::SigningError(e.to_string()))?);
        Ok(Self { signer })
    }

    pub fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), SignatureError> {
        self.signer
            .keypair()
            .map_err(|e| SignatureError::SigningError(e.to_string()))
    }

    pub fn sign(&self, message: &[u8], private_key: &[u8]) -> Result<Vec<u8>, SignatureError> {
        self.signer
            .sign(message, private_key)
            .map_err(|e| SignatureError::SigningError(e.to_string()))
    }

    pub fn verify(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool, SignatureError> {
        self.signer
            .verify(message, signature, public_key)
            .map_err(|e| SignatureError::VerificationError(e.to_string()))
    }
}
