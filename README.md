# 🔐 Quantum-Safe VPN with WebSocket Transport

A modern VPN implementation featuring **Post-Quantum Cryptography** protection with a **Rust server** and **React web client**. Route web traffic through encrypted tunnels between different IP addresses with quantum-resistant security.

## ⚡ Quick Start

### Prerequisites
- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Node.js 18+** - [Install Node.js](https://nodejs.org/)
- **Modern Browser** - Chrome, Firefox, Safari, or Edge

### 1. Start the Server
```bash
cd Server
cargo run --release
```
✅ Server runs on `0.0.0.0:8000`

### 2. Start the Client
```bash
cd Client
npm install
npm run dev
```
✅ Client available at `http://localhost:5173`

### 3. Connect & Use
1. Open web interface in your browser
2. Enter a username (any identifier)
3. Set server URL (e.g., `ws://REMOTE_IP:8000/vpn` for different IPs)
4. Click **"Connect"** and wait for green status
5. Toggle **"Route Web Traffic Through VPN"** to enable secure routing

## ✨ Features

### �️ Post-Quantum Security
- **Kyber768** - Quantum-resistant key exchange
- **Dilithium2** - Quantum-resistant digital signatures  
- **AES-256-GCM** - Authenticated symmetric encryption
- **Perfect Forward Secrecy** - Ephemeral keys for each session

### 🌐 Network Capabilities
- **WebSocket VPN Tunnel** - Encrypted traffic routing between different IPs
- **HTTP Proxy Support** - Routes web requests through secure tunnel
- **Real-time Statistics** - Live monitoring of latency, bandwidth, server load
- **Multi-client Support** - Concurrent connections with session management
- **Cross-platform** - Works on Windows, macOS, Linux

### 💻 Modern Interface
- **React + TypeScript** client with Tailwind CSS styling
- **One-click Traffic Toggle** - Simple enable/disable VPN routing
- **Live Connection Stats** - Real-time latency, bandwidth, and server metrics
- **Activity Logging** - Collapsible log with connection events and status
- **Responsive Design** - Works on desktop and mobile browsers

## �️ How It Works

### Connection Architecture
```
[Your Browser] ←→ [React Client] ←→ [Encrypted WebSocket] ←→ [Rust Server] ←→ [Internet]
    Local PC         VPN Interface        Quantum-Safe          Proxy Server      Target Sites
                                         Tunnel (Port 8000)
```

### Traffic Flow
1. **Secure Handshake** - Post-quantum key exchange (Kyber768 + Dilithium2)
2. **Toggle Activation** - User enables traffic routing with simple switch
3. **Request Interception** - HTTP requests captured and encrypted
4. **Tunnel Transport** - Encrypted data sent through WebSocket tunnel
5. **Server Proxy** - Server forwards requests to internet and returns responses
6. **Encrypted Response** - Data returned through secure tunnel to client

### Security Benefits
- **Quantum Protection** - Resistant to future quantum computing attacks
- **IP Masking** - Your real IP address is hidden from websites
- **End-to-End Encryption** - All tunnel traffic is encrypted
- **Geographic Routing** - Route through servers in different locations
- **Session Security** - Automatic cleanup and timeout protection

## 📁 Project Structure

```
PQC-VPN/
├── Server/                    # Rust VPN Server
│   ├── src/
│   │   ├── main.rs           # WebSocket server & HTTP proxy handler
│   │   ├── crypto.rs         # Post-quantum cryptography implementation
│   │   └── session.rs        # Client session management
│   ├── Cargo.toml            # Rust dependencies (Actix-Web, PQCrypto, etc.)
│   └── target/               # Compiled binaries
├── Client/                    # React Web Client
│   ├── src/
│   │   ├── App.tsx           # Main VPN interface with traffic toggle
│   │   ├── main.tsx          # React application entry point
│   │   └── index.css         # Tailwind CSS styles
│   ├── package.json          # Node.js dependencies (React, Vite, etc.)
│   └── dist/                 # Production build files
└── README.md                 # This documentation
```

## 🔧 Configuration & Deployment

### Local Development
```bash
# Terminal 1 - Start server
cd Server && cargo run

# Terminal 2 - Start client  
cd Client && npm run dev

# Open http://localhost:5173 in browser
```

### Different IP Addresses Setup
For client and server on different machines:

1. **Deploy Server** on remote machine:
   ```bash
   cd Server
   cargo build --release
   ./target/release/quantum-vpn-server
   ```

2. **Configure Client** to connect to remote server:
   - Server URL: `ws://REMOTE_IP:8000/vpn`
   - Ensure port 8000 is open in firewall

3. **Network Requirements**:
   - Open port 8000 on server machine
   - Ensure WebSocket connections are allowed
   - Consider using reverse proxy (nginx) for production

### Production Deployment

#### Server (Rust)
```bash
# Build optimized binary
cargo build --release

# Run with systemd service (recommended)
sudo systemctl enable quantum-vpn-server
sudo systemctl start quantum-vpn-server
```

#### Client (React)
```bash
# Build production assets
npm run build

# Serve with nginx, Apache, or any static file server
# Files will be in ./dist directory
```

#### Security Recommendations
- **Use HTTPS/WSS** - Deploy with TLS certificates for encrypted WebSocket
- **Firewall Configuration** - Restrict access to necessary ports only
- **Load Balancing** - Use nginx reverse proxy for scaling and SSL termination
- **Monitoring** - Implement logging, health checks, and performance monitoring

## 🔍 API Reference

### WebSocket Connection
- **Endpoint**: `ws://SERVER_IP:8000/vpn`
- **Protocol**: WebSocket with JSON message format

### Message Types

#### Client → Server
```json
{
  "type": "auth",
  "username": "user123"
}

{
  "type": "http_proxy_request",
  "id": "req_uuid", 
  "method": "GET",
  "url": "https://example.com",
  "headers": {"Accept": "text/html"}
}
```

#### Server → Client
```json
{
  "type": "auth_success",
  "server_info": {
    "name": "Quantum VPN Server",
    "encryption": "Post-Quantum (Kyber768 + Dilithium2)",
    "ip_address": "192.168.1.100",
    "port": "8000"
  }
}

{
  "type": "stats",
  "latency": 45,
  "bytes": {"rx": 1024, "tx": 2048},
  "server_load": 25,
  "connected_users": 1
}
```

## 🎯 Use Cases

- **🔒 Secure Browsing** - Protect web traffic with quantum-safe encryption
- **🌍 Geographic Routing** - Route traffic through servers in different locations
- **🛡️ Privacy Protection** - Hide your real IP address from websites
- **🔬 Development Testing** - Test applications with different server IPs
- **📚 Educational** - Learn VPN implementation and post-quantum cryptography
- **🔍 Network Research** - Study quantum-resistant network protocols

## 🚨 Troubleshooting

### Connection Issues
- ✅ Verify server is running: `netstat -an | findstr :8000`
- ✅ Check firewall allows port 8000
- ✅ Ensure WebSocket connections aren't blocked
- ✅ Try `ws://localhost:8000/vpn` for local testing

### Performance Issues
- 🔧 Large files may transfer slowly (WebSocket overhead)
- 🔧 High latency affects real-time performance
- 🔧 Consider server location for optimal routing

### Browser Compatibility
- ✅ Modern browsers support WebSockets
- ✅ Some corporate firewalls block WebSocket connections
- ✅ Try different browser if issues persist

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. **Fork** the repository on GitHub
2. **Clone** your fork: `git clone https://github.com/YOUR_USERNAME/PQC-VPN.git`
3. **Create** a feature branch: `git checkout -b feature/amazing-feature`
4. **Make** your changes and test thoroughly
5. **Commit** with clear messages: `git commit -m 'Add amazing feature'`
6. **Push** to your branch: `git push origin feature/amazing-feature`
7. **Open** a Pull Request with detailed description

### Development Guidelines
- Follow Rust and TypeScript best practices
- Add tests for new functionality
- Update documentation for any API changes
- Ensure cross-platform compatibility

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **[PQCrypto](https://pqcrypto.org/)** - Post-quantum cryptography implementations
- **[Actix-Web](https://actix.rs/)** - Fast and powerful Rust web framework
- **[React](https://reactjs.org/)** + **[Vite](https://vitejs.dev/)** - Modern web development tools
- **[Tailwind CSS](https://tailwindcss.com/)** - Utility-first CSS framework
- **Quantum cryptography research community** - For advancing quantum-safe protocols

---

**🚀 Ready to experience quantum-safe VPN protection?**

Get started by running the server and client, then toggle traffic routing to secure your connection with post-quantum encryption!

## 🏗️ How It Works

### Connection Flow
```
[Client Browser] ←→ [React App] ←→ [WebSocket Tunnel] ←→ [Rust Server] ←→ [Internet]
      ↓                ↓              ↓ Encrypted ↓           ↓
  User Interface   JSON Messages   Kyber768+AES256      HTTP Requests
```

1. **Establish Connection**: Client connects to server via WebSocket
2. **Authentication**: Simple username-based authentication
3. **Key Exchange**: Post-quantum key exchange (Kyber768 + Dilithium2)
4. **Traffic Routing**: Toggle enables routing of web traffic through encrypted tunnel
5. **HTTP Proxy**: Server forwards HTTP requests to internet and returns responses

### Key Components

#### Server (Rust)
- **main.rs** - WebSocket server, HTTP proxy handling, session management
- **crypto.rs** - Post-quantum cryptography (Kyber768, Dilithium2, AES-256-GCM)
- **session.rs** - Client session management with automatic cleanup

#### Client (React + TypeScript)
- **Modern UI** - Glass-morphism design with Tailwind CSS
- **Traffic Control** - Toggle switch for enabling VPN routing
- **Real-time Stats** - Live connection monitoring and statistics
- **Activity Logs** - Collapsible dropdown with connection events

## ⚙️ Configuration

### Different IP Addresses Setup
For client and server on different IPs:

1. **Deploy Server** on remote machine with public IP
2. **Update Client** server URL to `ws://REMOTE_IP:8000/vpn`
3. **Firewall** - Open port 8000 on server
4. **Connect** - Client will route traffic through remote server

### Environment Variables

#### Server
```bash
RUST_LOG=info              # Log level (debug, info, warn, error)
```

#### Client
Configure directly in the web interface:
- **Server URL**: WebSocket endpoint (ws://IP:8000/vpn)
- **Username**: Any identifier for the session

## 🔧 Production Deployment

### Server Setup
```bash
# Build optimized server
cd Server
cargo build --release

# Run server (consider using systemd service)
./target/release/quantum-vpn-server
```

### Client Deployment
```bash
# Build production client
cd Client
npm run build

# Serve with nginx, Apache, or any static file server
# Files will be in ./dist directory
```

### Security Recommendations
- **Use HTTPS/WSS** - Deploy with TLS certificates
- **Firewall Configuration** - Restrict access to port 8000
- **Load Balancing** - Use nginx reverse proxy for scaling
- **Monitoring** - Implement logging and health checks

## 🔐 Security Architecture

### Cryptographic Protection
1. **Post-Quantum Cryptography**:
   - **Kyber768** - Key exchange resistant to quantum computing attacks
   - **Dilithium2** - Digital signatures safe from quantum decryption
   - **AES-256-GCM** - Authenticated encryption for data protection

2. **Security Features**:
   - End-to-end encryption through WebSocket tunnel
   - Perfect forward secrecy with ephemeral keys
   - Automatic session timeout and cleanup
   - Real-time traffic monitoring and logging

3. **Network Security**:
   - Encrypted WebSocket transport (upgrade to WSS for production)
   - HTTP request/response proxying through secure tunnel
   - Session-based connection management

## 🌐 API Reference

### WebSocket Messages

#### Client → Server
```json
{
  "type": "auth",
  "username": "user123"
}

{
  "type": "http_proxy_request", 
  "id": "req_12345",
  "method": "GET",
  "url": "https://example.com",
  "headers": {"Accept": "text/html"}
}
```

#### Server → Client
```json
{
  "type": "auth_success",
  "server_info": {
    "name": "Quantum VPN Server",
    "location": "Global", 
    "encryption": "Post-Quantum (Kyber768 + Dilithium2)",
    "ip_address": "192.168.1.100",
    "port": "8000"
  }
}

{
  "type": "stats",
  "latency": 45,
  "bytes": {"rx": 1024, "tx": 2048},
  "connected_users": 1,
  "server_load": 25
}
```

## 📁 Project Structure

```
PQC-VPN/
├── Server/                          # Rust VPN Server
│   ├── src/
│   │   ├── main.rs                 # WebSocket server & HTTP proxy
│   │   ├── crypto.rs               # Post-quantum cryptography
│   │   └── session.rs              # Session management
│   ├── Cargo.toml                  # Rust dependencies
│   └── target/                     # Build artifacts
├── Client/                          # React Web Client  
│   ├── src/
│   │   ├── App.tsx                 # Main VPN interface
│   │   ├── main.tsx                # React entry point
│   │   └── index.css               # Tailwind CSS
│   ├── package.json                # Node.js dependencies
│   └── dist/                       # Built client files
├── README.md                       # Main documentation
└── VPN-PROXY-USAGE.md             # Detailed usage guide
```

## 🎯 Use Cases

- **Secure Browsing** - Encrypt web traffic through quantum-safe tunnel
- **Geographic Routing** - Route traffic through different server locations  
- **Development Testing** - Test applications with different IP addresses
- **Privacy Protection** - Hide client IP address from websites
- **Network Research** - Study post-quantum cryptography in practice
- **Educational** - Learn about VPN implementation and quantum-safe crypto

## 🤝 Contributing

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Setup
```bash
# Clone the repository
git clone https://github.com/Leonardo1903/PQC-VPN.git
cd PQC-VPN

# Start server (Terminal 1)
cd Server && cargo run

# Start client (Terminal 2) 
cd Client && npm run dev
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **PQCrypto** - Post-quantum cryptography implementations
- **Actix-Web** - Rust web framework for WebSocket server
- **React** + **Vite** - Modern web client framework
- **Tailwind CSS** - Utility-first CSS framework

---

**⚡ Ready to secure your connection with quantum-safe encryption?**

Start by running the server and client, then toggle the traffic routing to experience post-quantum protected VPN!
