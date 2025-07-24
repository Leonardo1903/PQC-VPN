use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::Message;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::time::Duration;

mod crypto;
mod session;

use crypto::KeyExchange;
use session::SessionManager;

#[derive(Debug, Serialize, Deserialize)]
struct AuthRequest {
    #[serde(rename = "type")]
    message_type: String,
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HandshakeRequest {
    client_public_key: Vec<u8>,
    signature: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct VpnPacket {
    #[serde(rename = "type")]
    packet_type: String,
    data: Vec<u8>,
    destination: Option<String>,
    protocol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StatsRequest {
    #[serde(rename = "type")]
    message_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerListRequest {
    #[serde(rename = "type")]
    message_type: String,
}

async fn handle_ws_connection(
    req: HttpRequest,
    stream: web::Payload,
    session_manager: web::Data<SessionManager>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;
    let peer_addr = req
        .peer_addr()
        .map(|addr| addr.to_string())
        .unwrap_or_default();

    // Get server's local IP address
    let server_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());

    let key_exchange = KeyExchange::new();
    let (kyber_public_key, dilithium_public_key) = key_exchange.get_public_keys();

    // Send server's public keys
    let initial_message = serde_json::json!({
        "kyber_public_key": kyber_public_key,
        "dilithium_public_key": dilithium_public_key,
    });

    let _ = session
        .text(serde_json::to_string(&initial_message).unwrap())
        .await;

    // Use actix_rt::spawn for non-Send futures
    actix_rt::spawn(async move {
        let mut session_id: Option<String> = None;
        let mut bytes_rx = 0u64;
        let mut bytes_tx = 0u64;
        let last_ping = std::time::Instant::now();

        while let Some(msg) = msg_stream.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // Try to parse as auth request first
                    if let Ok(auth_req) = serde_json::from_str::<AuthRequest>(&text) {
                        if auth_req.message_type == "auth" {
                            // Simple auth success response
                            let response = serde_json::json!({
                                "type": "auth_success",
                                "message": "Authentication successful",
                                "server_info": {
                                    "name": "Quantum VPN Server",
                                    "location": "Global",
                                    "encryption": "Post-Quantum (Kyber768 + Dilithium2)",
                                    "ip_address": server_ip,
                                    "port": "8000"
                                }
                            });

                            if let Err(e) = session
                                .text(serde_json::to_string(&response).unwrap())
                                .await
                            {
                                log::error!("Failed to send auth response: {}", e);
                                break;
                            }

                            // Start sending periodic stats
                            let mut session_clone = session.clone();
                            tokio::spawn(async move {
                                let mut interval = tokio::time::interval(Duration::from_secs(5));
                                loop {
                                    interval.tick().await;
                                    let stats = serde_json::json!({
                                        "type": "stats",
                                        "latency": rand::random::<u32>() % 50 + 10, // Simulated latency
                                        "bytes": {
                                            "rx": bytes_rx,
                                            "tx": bytes_tx
                                        },
                                        "connected_users": 1,
                                        "server_load": rand::random::<u32>() % 30 + 20
                                    });

                                    if session_clone
                                        .text(serde_json::to_string(&stats).unwrap())
                                        .await
                                        .is_err()
                                    {
                                        break;
                                    }
                                }
                            });
                            continue;
                        }
                    }

                    // Handle stats request
                    if let Ok(stats_req) = serde_json::from_str::<StatsRequest>(&text) {
                        if stats_req.message_type == "get_stats" {
                            let response = serde_json::json!({
                                "type": "stats_response",
                                "latency": (std::time::Instant::now() - last_ping).as_millis() as u32,
                                "bytes": {
                                    "rx": bytes_rx,
                                    "tx": bytes_tx
                                },
                                "uptime": "Connected",
                                "server_load": rand::random::<u32>() % 30 + 20
                            });

                            if let Err(e) = session
                                .text(serde_json::to_string(&response).unwrap())
                                .await
                            {
                                log::error!("Failed to send stats response: {}", e);
                            }
                            continue;
                        }
                    }

                    // Handle VPN packet tunneling
                    if let Ok(vpn_packet) = serde_json::from_str::<VpnPacket>(&text) {
                        if vpn_packet.packet_type == "tunnel_data" {
                            bytes_rx += vpn_packet.data.len() as u64;

                            // Simulate packet processing and forwarding
                            log::debug!(
                                "Processing VPN packet of {} bytes to {:?}",
                                vpn_packet.data.len(),
                                vpn_packet.destination
                            );

                            // Echo back processed data (in real implementation, forward to destination)
                            let response_packet = serde_json::json!({
                                "type": "tunnel_response",
                                "data": vpn_packet.data,
                                "processed": true
                            });

                            bytes_tx += vpn_packet.data.len() as u64;

                            if let Err(e) = session
                                .text(serde_json::to_string(&response_packet).unwrap())
                                .await
                            {
                                log::error!("Failed to send tunnel response: {}", e);
                            }
                            continue;
                        }
                    }

                    // Try to parse as handshake request
                    if let Ok(handshake) = serde_json::from_str::<HandshakeRequest>(&text) {
                        // Verify client's signature
                        if key_exchange
                            .verify_client_signature(
                                &handshake.client_public_key,
                                &handshake.signature,
                                &handshake.client_public_key,
                            )
                            .is_ok()
                        {
                            // Process client's public key and generate shared secret
                            if let Ok(shared_secret) =
                                key_exchange.process_client_key(&handshake.client_public_key)
                            {
                                // Create session with the shared secret
                                if let Ok(id) = session_manager.create_session(
                                    peer_addr.clone(),
                                    shared_secret,
                                    session.clone(),
                                ) {
                                    session_id = Some(id);
                                    log::info!("Session established for {}", peer_addr);
                                }
                            }
                        }
                    }
                }
                Ok(Message::Binary(data)) => {
                    if let Some(id) = &session_id {
                        if let Some(mut vpn_session) = session_manager.get_session(id) {
                            if let Ok(_decrypted) = vpn_session.crypto.decrypt(&data) {
                                // Handle decrypted VPN traffic here
                                log::debug!("Received {} bytes of encrypted data", data.len());
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    if let Some(id) = &session_id {
                        session_manager.remove_session(id);
                    }
                    break;
                }
                Err(e) => {
                    log::error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(response)
}

fn get_local_ip() -> Option<String> {
    use std::net::TcpStream;

    // Try to connect to a remote address to determine local IP
    if let Ok(stream) = TcpStream::connect("8.8.8.8:80") {
        if let Ok(local_addr) = stream.local_addr() {
            return Some(local_addr.ip().to_string());
        }
    }

    // Fallback: try to get local network interfaces
    None
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let session_manager = web::Data::new(SessionManager::new());
    let session_manager_cleanup = session_manager.clone();

    // Cleanup inactive sessions periodically
    tokio::spawn(async move {
        let cleanup_interval = Duration::from_secs(300); // 5 minutes
        let session_timeout = Duration::from_secs(3600); // 1 hour

        loop {
            tokio::time::sleep(cleanup_interval).await;
            session_manager_cleanup.cleanup_inactive_sessions(session_timeout);
        }
    });

    log::info!("Starting VPN server on 0.0.0.0:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(session_manager.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/vpn").route(web::get().to(handle_ws_connection)))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
