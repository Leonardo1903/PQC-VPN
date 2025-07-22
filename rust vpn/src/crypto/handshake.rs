use std::sync::Arc;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};
use blake2::{Blake2s256, Digest};
use hkdf::Hkdf;
use thiserror::Error;

use super::{CryptoProvider, CryptoError};

const HANDSHAKE_MESSAGE_SIZE: usize = 1232;
const MAC_SIZE: usize = 32;
const TIMESTAMP_SIZE: usize = 12;
const RESERVED_SIZE: usize = 3;
const INDEX_SIZE: usize = 4;

#[derive(Debug, Error)]
pub enum HandshakeError {
    #[error("Crypto provider error: {0}")]
    CryptoError(#[from] CryptoError),
    #[error("Invalid message size")]
    InvalidMessageSize,
    #[error("Invalid MAC")]
    InvalidMac,
    #[error("Invalid timestamp")]
    InvalidTimestamp,
}

#[derive(Debug)]
pub enum HandshakeState {
    InitiatorStart,
    InitiatorFinished,
    ResponderStart,
    ResponderFinished,
}

#[derive(Debug)]
pub struct HandshakeMessage {
    message_type: u8,
    sender_index: u32,
    ephemeral_public: Vec<u8>,
    static_ciphertext: Vec<u8>,
    timestamp: Vec<u8>,
    mac1: Vec<u8>,
    mac2: Vec<u8>,
}

impl HandshakeMessage {
    pub fn new(
        message_type: u8,
        sender_index: u32,
        ephemeral_public: Vec<u8>,
        static_ciphertext: Vec<u8>,
        timestamp: Vec<u8>,
    ) -> Self {
        Self {
            message_type,
            sender_index,
            ephemeral_public,
            static_ciphertext,
            timestamp,
            mac1: vec![0; MAC_SIZE],
            mac2: vec![0; MAC_SIZE],
        }
    }

    pub fn serialize(&self) -> Result<Bytes, HandshakeError> {
        let mut buf = BytesMut::with_capacity(HANDSHAKE_MESSAGE_SIZE);
        
        buf.put_u8(self.message_type);
        buf.put_bytes(0, RESERVED_SIZE);
        buf.put_u32_le(self.sender_index);
        buf.put_slice(&self.ephemeral_public);
        buf.put_slice(&self.static_ciphertext);
        buf.put_slice(&self.timestamp);
        buf.put_slice(&self.mac1);
        buf.put_slice(&self.mac2);
        
        if buf.len() > HANDSHAKE_MESSAGE_SIZE {
            return Err(HandshakeError::InvalidMessageSize);
        }
        
        Ok(buf.freeze())
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, HandshakeError> {
        if bytes.len() != HANDSHAKE_MESSAGE_SIZE {
            return Err(HandshakeError::InvalidMessageSize);
        }
        
        let mut buf = Bytes::copy_from_slice(bytes);
        
        let message_type = buf.get_u8();
        buf.advance(RESERVED_SIZE);
        let sender_index = buf.get_u32_le();
        
        let ephemeral_size = 896; // Kyber768 public key size
        let static_size = 188;    // Classic McEliece ciphertext size
        
        let ephemeral_public = buf.copy_to_bytes(ephemeral_size).to_vec();
        let static_ciphertext = buf.copy_to_bytes(static_size).to_vec();
        let timestamp = buf.copy_to_bytes(TIMESTAMP_SIZE).to_vec();
        let mac1 = buf.copy_to_bytes(MAC_SIZE).to_vec();
        let mac2 = buf.copy_to_bytes(MAC_SIZE).to_vec();
        
        Ok(Self {
            message_type,
            sender_index,
            ephemeral_public,
            static_ciphertext,
            timestamp,
            mac1,
            mac2,
        })
    }
}

pub struct Handshake<P: CryptoProvider> {
    state: HandshakeState,
    crypto_provider: Arc<P>,
    static_private: Vec<u8>,
    static_public: Vec<u8>,
    ephemeral_private: Option<Vec<u8>>,
    shared_secret: Option<Vec<u8>>,
}

impl<P: CryptoProvider> Handshake<P> {
    pub fn new(crypto_provider: Arc<P>, is_initiator: bool) -> Result<Self, HandshakeError> {
        let (static_public, static_private) = crypto_provider.generate_static_keypair()?;
        
        Ok(Self {
            state: if is_initiator {
                HandshakeState::InitiatorStart
            } else {
                HandshakeState::ResponderStart
            },
            crypto_provider,
            static_private,
            static_public,
            ephemeral_private: None,
            shared_secret: None,
        })
    }
    
    pub fn create_initiation(&mut self) -> Result<HandshakeMessage, HandshakeError> {
        if !matches!(self.state, HandshakeState::InitiatorStart) {
            return Err(HandshakeError::InvalidMessageSize);
        }
        
        // Generate ephemeral keypair
        let (ephemeral_public, ephemeral_private) = self.crypto_provider.generate_ephemeral_keypair()?;
        self.ephemeral_private = Some(ephemeral_private);
        
        // Encapsulate with responder's static key
        let (shared_secret, static_ciphertext) = self.crypto_provider.encapsulate_static(&self.static_public)?;
        self.shared_secret = Some(shared_secret);
        
        // Create timestamp
        let timestamp = vec![0; TIMESTAMP_SIZE]; // TODO: Implement actual timestamp
        
        Ok(HandshakeMessage::new(1, 0, ephemeral_public, static_ciphertext, timestamp))
    }
    
    pub fn process_initiation(&mut self, msg: HandshakeMessage) -> Result<HandshakeMessage, HandshakeError> {
        if !matches!(self.state, HandshakeState::ResponderStart) {
            return Err(HandshakeError::InvalidMessageSize);
        }
        
        // Decapsulate static secret
        let shared_secret = self.crypto_provider.decapsulate_static(
            &msg.static_ciphertext,
            &self.static_private
        )?;
        
        // Generate response
        let (ephemeral_public, ephemeral_private) = self.crypto_provider.generate_ephemeral_keypair()?;
        self.ephemeral_private = Some(ephemeral_private);
        
        // Encapsulate with initiator's ephemeral key
        let (ephemeral_ss, static_ciphertext) = self.crypto_provider.encapsulate_ephemeral(&msg.ephemeral_public)?;
        
        // Combine shared secrets
        let mut combined_secret = Vec::with_capacity(shared_secret.len() + ephemeral_ss.len());
        combined_secret.extend_from_slice(&shared_secret);
        combined_secret.extend_from_slice(&ephemeral_ss);
        
        self.shared_secret = Some(combined_secret);
        self.state = HandshakeState::ResponderFinished;
        
        // Create timestamp
        let timestamp = vec![0; TIMESTAMP_SIZE]; // TODO: Implement actual timestamp
        
        Ok(HandshakeMessage::new(2, 1, ephemeral_public, static_ciphertext, timestamp))
    }
    
    pub fn process_response(&mut self, msg: HandshakeMessage) -> Result<(), HandshakeError> {
        if !matches!(self.state, HandshakeState::InitiatorStart) {
            return Err(HandshakeError::InvalidMessageSize);
        }
        
        // Decapsulate ephemeral secret
        let ephemeral_ss = self.crypto_provider.decapsulate_ephemeral(
            &msg.static_ciphertext,
            self.ephemeral_private.as_ref().unwrap()
        )?;
        
        // Combine shared secrets
        let mut combined_secret = Vec::new();
        combined_secret.extend_from_slice(self.shared_secret.as_ref().unwrap());
        combined_secret.extend_from_slice(&ephemeral_ss);
        
        self.shared_secret = Some(combined_secret);
        self.state = HandshakeState::InitiatorFinished;
        
        Ok(())
    }
    
    pub fn get_shared_secret(&self) -> Option<&[u8]> {
        self.shared_secret.as_deref()
    }
}
