use std::sync::Arc;
use clap::Parser;
use tokio::sync::mpsc;
use tracing::{info, warn, error};

use crate::{
    config::{Config, InterfaceConfig},
    crypto::{PqcWireguardCryptoProvider, KemMode},
    network::{TunDevice, Peer, PeerConfig, CryptokeyRoutingTable},
};

mod config;
mod crypto;
mod network;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long)]
    config: String,
    
    #[arg(long)]
    mode: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Load configuration
    let config = Config::load(&cli.config)?;
    
    // Determine KEM mode
    let kem_mode = match cli.mode.as_deref() {
        Some("pqc") => KemMode::PqcOnly,
        Some("hybrid") => KemMode::Hybrid,
        Some("classical") => KemMode::Classical,
        _ => config.interface.kem_mode,
    };
    
    info!("Starting PQC-VPN with {:?} mode", kem_mode);
    
    // Initialize crypto provider
    let crypto_provider = Arc::new(PqcWireguardCryptoProvider::new()?);
    
    // Create TUN device
    let mut tun = TunDevice::new(
        &config.interface.name,
        config.interface.mtu.unwrap_or(1420),
    )?;
    
    info!("Created TUN device {}", config.interface.name);
    
    // Initialize routing table
    let mut routing_table = CryptokeyRoutingTable::new();
    
    // Set up peer communication channels
    let (tx, rx) = mpsc::channel(32);
    
    // Initialize peers
    let mut peers = Vec::new();
    
    for peer_config in config.peers {
        let peer = Peer::new(
            PeerConfig {
                endpoint: peer_config.endpoint.unwrap(),
                allowed_ips: peer_config.allowed_ips,
                keepalive_interval: peer_config.persistent_keepalive,
                public_key: hex::decode(&peer_config.public_key)?,
            },
            crypto_provider.clone(),
            rx.clone(),
            tx.clone(),
        );
        
        peers.push(peer);
    }
    
    info!("Initialized {} peers", peers.len());
    
    // Main event loop
    loop {
        tokio::select! {
            Some(packet) = tun.read_packet() => {
                // Process incoming packets
                if let Some(peer_key) = routing_table.find_peer(packet.get_bytes()[0].into()) {
                    // Route packet to appropriate peer
                }
            }
            else => {
                error!("TUN device error");
                break;
            }
        }
    }
    
    Ok(())
}
