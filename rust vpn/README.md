# PQC-VPN

A Post-Quantum Cryptography VPN implementation based on WireGuard, using the Open Quantum Safe library for post-quantum cryptographic operations.

## Features

- Post-quantum cryptography using OQS library
- Classic McEliece for static key encapsulation
- Kyber768-Dagger for ephemeral key encapsulation
- Dilithium2 for digital signatures (with Falcon-512 and SPHINCS+ fallback)
- ChaCha20Poly1305 for symmetric encryption
- MTU-constrained handshake (1232 bytes)
- Hybrid mode combining classical and PQC algorithms
- Comprehensive benchmarking suite

## Requirements

- Rust 1.70 or later
- Cargo
- LLVM/Clang (required for bindgen)
- CMake (for building OQS dependencies)
- OpenSSL development libraries

### Windows Setup

Open PowerShell as Administrator and follow these steps:

1. Install LLVM:
   ```powershell
   winget install LLVM.LLVM
   ```

2. Install CMake:
   ```powershell
   winget install Kitware.CMake
   ```

3. Install Chocolatey (if not already installed):
   ```powershell
   Set-ExecutionPolicy Bypass -Scope Process -Force
   [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
   iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
   ```

4. Install OpenSSL:
   ```powershell
   choco install openssl -y
   ```

5. Set required environment variables (you can add these to your system environment variables for persistence):
   ```powershell
   $env:Path += ";C:\Program Files\LLVM\bin"
   $env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin\libclang.dll"
   $env:OPENSSL_ROOT_DIR = "C:\Program Files\OpenSSL-Win64"
   ```

### Linux Setup

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install llvm clang libclang-dev cmake build-essential

# Fedora
sudo dnf install llvm clang clang-devel cmake

# Arch Linux
sudo pacman -S llvm clang cmake base-devel
```

## Installation

1. Clone the repository
2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

1. Create a configuration file (example in `config/example.conf`)
2. Run the VPN:
   ```bash
   pqc-vpn --config /path/to/config.conf
   ```

### Modes

The VPN supports three operational modes:
- PQC-only: Uses only post-quantum algorithms
- Hybrid: Combines classical (Curve25519) with PQC
- Classical: Uses only classical algorithms (for testing/comparison)

Select the mode using the `--mode` flag:
```bash
pqc-vpn --config config.conf --mode pqc    # PQC-only mode
pqc-vpn --config config.conf --mode hybrid # Hybrid mode
```

## Benchmarking

Run the benchmark suite:
```bash
cargo bench
```

This will test:
- Handshake latency
- Throughput
- CPU/Memory usage
- Decryption Failure Rate (DFR)

## Architecture

The implementation follows a modular design:

- `crypto/`: Cryptographic operations
  - `provider.rs`: Trait for crypto operations
  - `handshake.rs`: Post-quantum handshake protocol
  - `kex.rs`: Key exchange implementations
  - `aead.rs`: ChaCha20Poly1305 wrapper
  - `signature.rs`: Digital signature schemes

- `network/`: Networking components
  - `device.rs`: TUN device interface
  - `peer.rs`: Peer management
  - `routing.rs`: Cryptokey routing

- `config/`: Configuration handling
  - `settings.rs`: Config structures

## Security

This implementation uses:
- McEliece-460896 for IND-CCA security
- Kyber768-Dagger for IND-CPA security
- Dilithium2 for EUF-CMA security
- ChaCha20Poly1305 for AEAD
- BLAKE2s for hashing

## License

MIT OR Apache-2.0
