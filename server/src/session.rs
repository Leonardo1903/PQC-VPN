use crate::crypto::{CryptoError, CryptoSession};
use actix_ws::Session;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientInfo {
    pub id: String,
    pub ip: String,
    pub connected_at: SystemTime,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

pub struct VpnSession {
    pub id: String,
    pub client_info: ClientInfo,
    pub crypto: CryptoSession,
    pub ws: Session,
}

#[derive(Clone)]
pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<String, VpnSession>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_session(
        &self,
        ip: String,
        shared_key: Vec<u8>,
        ws: Session,
    ) -> Result<String, CryptoError> {
        let id = Uuid::new_v4().to_string();

        let client_info = ClientInfo {
            id: id.clone(),
            ip,
            connected_at: SystemTime::now(),
            bytes_sent: 0,
            bytes_received: 0,
        };

        let session = VpnSession {
            id: id.clone(),
            client_info,
            crypto: CryptoSession::new(shared_key)?,
            ws,
        };

        self.sessions.lock().unwrap().insert(id.clone(), session);
        Ok(id)
    }

    pub fn get_session(&self, id: &str) -> Option<VpnSession> {
        self.sessions.lock().unwrap().remove(id)
    }

    pub fn remove_session(&self, id: &str) {
        self.sessions.lock().unwrap().remove(id);
    }

    pub fn list_sessions(&self) -> Vec<ClientInfo> {
        self.sessions
            .lock()
            .unwrap()
            .values()
            .map(|s| s.client_info.clone())
            .collect()
    }

    pub fn cleanup_inactive_sessions(&self, timeout: Duration) {
        let now = SystemTime::now();
        self.sessions.lock().unwrap().retain(|_, session| {
            now.duration_since(session.client_info.connected_at)
                .unwrap_or_default()
                < timeout
        });
    }
}
