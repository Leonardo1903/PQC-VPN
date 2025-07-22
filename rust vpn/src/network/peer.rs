use std::sync::Arc;
use std::net::SocketAddr;
use ipnetwork::IpNetwork;
use tokio::sync::mpsc;
use thiserror::Error;

use crate::crypto::{CryptoProvider, HandshakeState, HandshakeMessage, KemMode};

#[derive(Debug, Error)]
pub enum PeerError {
    #[error("Handshake error")]
    HandshakeError,
    #[error("Invalid state")]
    InvalidState,
}

pub struct PeerConfig {
    pub endpoint: SocketAddr,
    pub allowed_ips: Vec<IpNetwork>,
    pub keepalive_interval: Option<u16>,
    pub public_key: Vec<u8>,
}

pub struct Peer<P: CryptoProvider> {
    config: PeerConfig,
    handshake_state: HandshakeState,
    crypto_provider: Arc<P>,
    rx: mpsc::Receiver<HandshakeMessage>,
    tx: mpsc::Sender<HandshakeMessage>,
}

impl<P: CryptoProvider> Peer<P> {
    pub fn new(
        config: PeerConfig,
        crypto_provider: Arc<P>,
        rx: mpsc::Receiver<HandshakeMessage>,
        tx: mpsc::Sender<HandshakeMessage>,
    ) -> Self {
        Self {
            config,
            handshake_state: HandshakeState::InitiatorStart,
            crypto_provider,
            rx,
            tx,
        }
    }

    pub async fn start_handshake(&mut self) -> Result<(), PeerError> {
        match self.handshake_state {
            HandshakeState::InitiatorStart => {
                let msg = self.create_initiation_message()?;
                self.tx.send(msg).await.map_err(|_| PeerError::HandshakeError)?;
                Ok(())
            }
            _ => Err(PeerError::InvalidState),
        }
    }

    fn create_initiation_message(&self) -> Result<HandshakeMessage, PeerError> {
        // Create handshake message...
        todo!()
    }

    pub async fn handle_handshake_message(&mut self, msg: HandshakeMessage) -> Result<(), PeerError> {
        match self.handshake_state {
            HandshakeState::ResponderStart => {
                // Process initiation and create response...
                todo!()
            }
            HandshakeState::InitiatorStart => {
                // Process response...
                todo!()
            }
            _ => Err(PeerError::InvalidState),
        }
    }
}
