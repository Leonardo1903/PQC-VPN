# Quantum-Safe VPN with WebSocket Transport

A production-grade VPN implementation using WebSockets and Post-Quantum Cryptography with a Rust server and React web client.

## Features

- 🔐 Post-Quantum Cryptography
  - Kyber for key exchange (Kyber768)
  - Dilithium for digital signatures
  - AES-256-GCM for symmetric encryption

- 🔒 Security Features
  - End-to-end encryption
  - Perfect forward secrecy
  - Kill switch functionality
  - JWT-based authentication for admin endpoints

- 🌐 Network Features
  - WebSocket-based tunnel
  - Multi-client support
  - Real-time data tunneling
  - IPv4 and IPv6 support
  - Server IP address display

- 📊 Monitoring & Features
  - Session monitoring
  - Traffic statistics
  - Real-time connection stats
  - Data tunneling test interface
  - Connection activity logging
  - Protected admin API

## Prerequisites

- Rust 1.70 or higher
- Node.js 18 or higher
- OpenSSL development libraries

## Building

### Server

```bash
cd Server
cargo build --release
```

### Web Client

```bash
cd Client
npm install
npm run build
```

## Running

### Server

```bash
cd Server
cargo run --release
```

The server will start on `0.0.0.0:8000` by default.

### Web Client

```bash
cd Client
npm run dev
```

For production:

```bash
cd Client
npm run build
# Serve the built files from the 'dist' directory
```

## Configuration

The server and client can be configured through environment variables:

### Server
- `RUST_LOG`: Log level (default: "info")
- `VPN_SERVER_PORT`: Server port (default: 8000)
- `SESSION_TIMEOUT`: Session timeout in seconds (default: 3600)
- `ADMIN_TOKEN`: JWT token for admin API access

### Web Client
- Configure the server URL directly in the web interface (default: ws://localhost:8000/vpn)
- Built with React + TypeScript + Vite + Tailwind CSS
- Real-time VPN statistics and monitoring
- Tunnel data testing interface
- Connection activity logging

## Security Considerations

1. The implementation uses post-quantum cryptography:
   - Kyber for key exchange (resistant to quantum computers)
   - Dilithium for digital signatures (quantum-resistant)
   - AES-256-GCM for symmetric encryption

2. All traffic is end-to-end encrypted with perfect forward secrecy

3. The server implements session management with automatic cleanup of inactive sessions

4. Admin endpoints are protected with JWT authentication

## Architecture

### Server Components (Rust)

- `crypto.rs`: Implements quantum-safe cryptography operations
- `session.rs`: Manages client sessions and connection state
- `main.rs`: WebSocket server and request handling

### Web Client Components (React + TypeScript)

- Modern React application with Vite and Tailwind CSS
- WebSocket client implementation with real-time communication
- Post-quantum cryptography handshake support
- Real-time connection status and statistics display
- Data tunneling test interface
- Connection activity logging
- Server information display (IP address, encryption details)
- Responsive design for desktop and mobile

## API Endpoints

### WebSocket Connection
- `ws://localhost:8000/vpn` - Main VPN WebSocket endpoint

### Admin API
- `GET /admin/sessions` - List active sessions (requires JWT authentication)

## Message Types

The WebSocket connection supports the following message types:

### Client to Server
- `auth` - Authentication request with username
- `tunnel_data` - Data to be tunneled through VPN
- `get_stats` - Request current statistics

### Server to Client
- `auth_success` - Authentication successful with server info
- `auth_error` - Authentication failed
- `stats` - Real-time statistics update
- `tunnel_response` - Response to tunneled data
- `error` - Error message

## Usage

1. **Start the Server**: Run the Rust server which will listen on port 8000
2. **Open the Client**: Access the web interface at http://localhost:5173 (or 5174)
3. **Connect**: Enter a username and click "Connect"
4. **Monitor**: View real-time statistics including latency, data transfer, and server load
5. **Test Tunneling**: Use the tunnel test interface to send data through the VPN
6. **View Logs**: Monitor connection activity in the built-in log viewer

## Quick Start

```bash
# Terminal 1 - Start the server
cd Server
cargo run

# Terminal 2 - Start the client
cd Client
npm run dev

# Open browser to http://localhost:5173 or 5174
```

## Project Structure

```
├── Server/                 # Rust VPN Server
│   ├── src/
│   │   ├── main.rs        # WebSocket server and request handling
│   │   ├── crypto.rs      # Post-quantum cryptography operations
│   │   └── session.rs     # Client session management
│   └── Cargo.toml         # Rust dependencies
├── Client/                 # React Web Client
│   ├── src/
│   │   ├── App.tsx        # Main VPN client interface
│   │   ├── main.tsx       # React app entry point
│   │   └── index.css      # Tailwind CSS styles
│   └── package.json       # Node.js dependencies
└── README.md              # Project documentation
```

## Production Deployment

For production deployment:

1. Use TLS termination (e.g., with nginx)
2. Set up proper logging and monitoring
3. Configure firewall rules
4. Use strong JWT secrets for admin API
5. Regular security audits and updates

## Contributing

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

## License

MIT License
