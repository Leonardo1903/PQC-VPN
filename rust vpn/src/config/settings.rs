use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use ipnetwork::IpNetwork;

use crate::crypto::KemMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceConfig {
    pub name: String,
    pub private_key: String,
    pub listen_port: u16,
    pub mtu: Option<i32>,
    pub fwmark: Option<u32>,
    pub kem_mode: KemMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerConfig {
    pub public_key: String,
    pub endpoint: Option<SocketAddr>,
    pub allowed_ips: Vec<IpNetwork>,
    pub persistent_keepalive: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub interface: InterfaceConfig,
    pub peers: Vec<PeerConfig>,
}
