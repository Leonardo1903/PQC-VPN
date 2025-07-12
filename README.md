# 🦀 VPN Proxy System

A simple VPN-like proxy system with a Rust backend and React frontend that allows you to browse websites through a proxy server.

## 🏗️ Architecture

- **Backend (Rust)**: Proxy server that fetches content from URLs on behalf of the client
- **Frontend (React)**: Web interface for entering URLs and viewing proxied content
- **Communication**: REST API between frontend and backend

## 📁 Project Structure

```
PQC-Vpn/
├── vpn-server/     # Rust backend (actix-web + reqwest)
├── vpn-client/     # React frontend (Vite + Tailwind CSS v4)
└── README.md
```

## 🚀 Quick Start

### Prerequisites

- **Rust** (latest stable version)
- **Node.js** (v18 or higher)
- **npm** or **yarn**

### 1. Clone the Repository

```bash
git clone <your-repo-url>
cd PQC-Vpn
```

### 2. Start the Rust Backend

```bash
cd vpn-server
cargo run
```

The server will start on `http://localhost:8888`

### 3. Start the React Frontend

In a new terminal:

```bash
cd vpn-client
npm install
npm run dev
```

The frontend will start on `http://localhost:5173`

### 4. Use the Application

1. Open your browser and go to `http://localhost:5173`
2. Enter any URL (e.g., `https://example.com`)
3. Click "Go" to fetch the content through the proxy
4. View the proxied content in the sandboxed iframe

## 🔧 Features

### Backend Features
- ✅ REST API endpoint `/proxy` for fetching URLs
- ✅ CORS enabled for frontend communication
- ✅ Custom user agent to avoid blocking
- ✅ Error handling and status codes
- ✅ Timeout protection (30 seconds)

### Frontend Features
- ✅ Modern React with hooks
- ✅ Tailwind CSS v4 for styling
- ✅ Loading states and error handling
- ✅ Sandboxed iframe for security
- ✅ Responsive design
- ✅ Status code display

## 🛠️ Technical Details

### Backend Dependencies
- `actix-web`: Web framework
- `actix-cors`: CORS middleware
- `reqwest`: HTTP client for fetching URLs
- `serde`: Serialization/deserialization
- `tokio`: Async runtime

### Frontend Dependencies
- `react`: UI framework
- `axios`: HTTP client for API calls
- `vite`: Build tool and dev server
- `@tailwindcss/vite`: Tailwind CSS v4 plugin

## 🔒 Security Notes

- The iframe is sandboxed to prevent XSS attacks
- CORS is configured to only allow the frontend origin
- The proxy server uses a custom user agent to avoid detection
- This is a basic proxy, not a full VPN with packet routing/tunneling

## 🚧 Development

### Backend Development

```bash
cd vpn-server
cargo build          # Build the project
cargo run            # Run in development mode
cargo test           # Run tests
```

### Frontend Development

```bash
cd vpn-client
npm install          # Install dependencies
npm run dev          # Start development server
npm run build        # Build for production
npm run preview      # Preview production build
```

## 🐛 Troubleshooting

### Common Issues

1. **"Failed to fetch the URL" error**
   - Make sure the Rust server is running on port 8888
   - Check that the URL is valid and accessible

2. **CORS errors**
   - Ensure the frontend is running on port 5173
   - Check that the backend CORS configuration matches

3. **Content not displaying**
   - Some websites may block iframe embedding
   - Try different URLs or check the browser console

4. **Port conflicts**
   - If ports 8888 or 5173 are already in use:
     - Backend: Modify the port in `vpn-server/src/main.rs`
     - Frontend: Modify the port in `vpn-client/vite.config.js`

### Debug Commands

```bash
# Check if backend is running
curl http://localhost:8888/proxy -X POST -H "Content-Type: application/json" -d '{"url":"https://example.com"}'

# Check frontend build
cd vpn-client && npm run build
```

## 📝 API Reference

### POST /proxy

Fetches content from a URL through the proxy.

**Request:**
```json
{
  "url": "https://example.com"
}
```

**Response:**
```json
{
  "html": "<!DOCTYPE html>...",
  "status": 200
}
```

## 🎨 Customization

### Styling
The frontend uses Tailwind CSS v4. You can customize the design by:
- Modifying classes in `vpn-client/src/App.jsx`
- Adding custom CSS in `vpn-client/src/index.css`
- Updating the Tailwind config in `vpn-client/tailwind.config.js`

### Backend Configuration
You can modify the backend behavior by:
- Changing the user agent in `vpn-server/src/main.rs`
- Adjusting timeout settings
- Adding custom headers or request modifications

## 🚀 Future Enhancements

- [ ] Add request/response header modification
- [ ] Implement IP anonymization
- [ ] Add support for different content types
- [ ] Implement caching
- [ ] Add authentication
- [ ] Real VPN functionality with tun interfaces

## 📄 License

This project is for educational purposes. Use responsibly and in accordance with applicable laws and terms of service.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📞 Support

If you encounter any issues or have questions:
1. Check the troubleshooting section above
2. Review the error messages in the browser console
3. Check the server logs for backend issues
4. Open an issue on GitHub with detailed information

---

**Happy proxying! 🦀** 