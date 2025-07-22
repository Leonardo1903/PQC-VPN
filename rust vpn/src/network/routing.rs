use std::collections::HashMap;
use ipnetwork::IpNetwork;
use std::net::IpAddr;

pub struct CryptokeyRoutingTable {
    routes: HashMap<IpNetwork, Vec<u8>>, // Maps IP networks to peer public keys
}

impl CryptokeyRoutingTable {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, network: IpNetwork, peer_key: Vec<u8>) {
        self.routes.insert(network, peer_key);
    }

    pub fn remove_route(&mut self, network: &IpNetwork) -> Option<Vec<u8>> {
        self.routes.remove(network)
    }

    pub fn find_peer(&self, addr: IpAddr) -> Option<&Vec<u8>> {
        self.routes
            .iter()
            .find(|(network, _)| network.contains(addr))
            .map(|(_, key)| key)
    }

    pub fn get_networks_for_peer(&self, peer_key: &[u8]) -> Vec<IpNetwork> {
        self.routes
            .iter()
            .filter(|(_, key)| key.as_slice() == peer_key)
            .map(|(network, _)| *network)
            .collect()
    }
}
