import { useState, useRef } from "react";

interface VpnState {
  connected: boolean;
  connecting: boolean;
  authenticated: boolean;
  error: string | null;
  latency: number | null;
  bytesTransferred: { rx: number; tx: number };
  serverInfo: {
    name: string;
    location: string;
    encryption: string;
    ip_address: string;
    port: string;
  } | null;
  serverLoad: number;
  connectedUsers: number;
}

interface ServerMessage {
  type: string;
  message?: string;
  server_info?: {
    name: string;
    location: string;
    encryption: string;
    ip_address: string;
    port: string;
  };
  latency?: number;
  bytes?: { rx: number; tx: number };
  connected_users?: number;
  server_load?: number;
}

function App() {
  const [vpnState, setVpnState] = useState<VpnState>({
    connected: false,
    connecting: false,
    authenticated: false,
    error: null,
    latency: null,
    bytesTransferred: { rx: 0, tx: 0 },
    serverInfo: null,
    serverLoad: 0,
    connectedUsers: 0,
  });

  const [serverUrl, setServerUrl] = useState("ws://localhost:8000/vpn");
  const [username, setUsername] = useState("");
  const [tunnelData, setTunnelData] = useState("");
  const [connectionLog, setConnectionLog] = useState<string[]>([]);
  const wsRef = useRef<WebSocket | null>(null);

  const connect = async () => {
    if (!username.trim()) {
      setVpnState((prev) => ({ ...prev, error: "Username is required" }));
      return;
    }

    setVpnState((prev) => ({
      ...prev,
      connecting: true,
      error: null,
    }));

    try {
      const ws = new WebSocket(serverUrl);
      wsRef.current = ws;

      ws.onopen = () => {
        console.log("WebSocket connected");
        // Send authentication request
        ws.send(
          JSON.stringify({
            type: "auth",
            username: username,
          })
        );
      };

      ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          handleServerMessage(data);
        } catch (e) {
          console.error("Failed to parse message:", e);
        }
      };

      ws.onclose = () => {
        console.log("WebSocket disconnected");
        setVpnState((prev) => ({
          ...prev,
          connected: false,
          connecting: false,
          authenticated: false,
        }));
      };

      ws.onerror = (error) => {
        console.error("WebSocket error:", error);
        setVpnState((prev) => ({
          ...prev,
          error: "Connection failed",
          connecting: false,
        }));
      };
    } catch {
      setVpnState((prev) => ({
        ...prev,
        error: "Failed to connect",
        connecting: false,
      }));
    }
  };

  const disconnect = () => {
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }
    setConnectionLog((prev) => [...prev, "Disconnected from VPN server"]);
  };

  const sendTunnelData = (data: string) => {
    if (wsRef.current && vpnState.connected) {
      const packet = {
        type: "tunnel_data",
        data: Array.from(new TextEncoder().encode(data)),
        destination: "internet",
        protocol: "http",
      };
      wsRef.current.send(JSON.stringify(packet));
      setConnectionLog((prev) => [
        ...prev,
        `Tunneled ${data.length} bytes of data`,
      ]);
    }
  };

  const clearLog = () => {
    setConnectionLog([]);
  };

  const handleServerMessage = (data: ServerMessage) => {
    switch (data.type) {
      case "auth_success":
        setVpnState((prev) => ({
          ...prev,
          authenticated: true,
          connecting: false,
          connected: true,
          error: null,
          serverInfo: data.server_info || null,
        }));
        setConnectionLog((prev) => [
          ...prev,
          `Connected to ${data.server_info?.name || "VPN Server"}`,
        ]);
        break;
      case "auth_error":
        setVpnState((prev) => ({
          ...prev,
          error: data.message || "Authentication failed",
          connecting: false,
        }));
        break;
      case "stats":
        setVpnState((prev) => ({
          ...prev,
          latency: data.latency || prev.latency,
          bytesTransferred: data.bytes || prev.bytesTransferred,
          connectedUsers: data.connected_users || prev.connectedUsers,
          serverLoad: data.server_load || prev.serverLoad,
        }));
        break;
      case "tunnel_response":
        // Handle tunneled data response
        setConnectionLog((prev) => [...prev, "Data tunneled successfully"]);
        break;
      case "error":
        setVpnState((prev) => ({
          ...prev,
          error: data.message || "Unknown error",
          connecting: false,
        }));
        break;
    }
  };

  const formatBytes = (bytes: number) => {
    const sizes = ["B", "KB", "MB", "GB"];
    if (bytes === 0) return "0 B";
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return Math.round((bytes / Math.pow(1024, i)) * 100) / 100 + " " + sizes[i];
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
      <div className="container mx-auto px-4 py-8">
        {/* Header */}
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-white mb-2">Quantum VPN</h1>
          <p className="text-slate-300">
            Post-Quantum Cryptography Protected VPN
          </p>
        </div>

        {/* Main Panel */}
        <div className="max-w-md mx-auto bg-white/10 backdrop-blur-lg rounded-2xl p-6 border border-white/20">
          {/* Connection Status */}
          <div className="text-center mb-6">
            <div
              className={`inline-flex items-center px-4 py-2 rounded-full text-sm font-medium ${
                vpnState.connected
                  ? "bg-green-500/20 text-green-300 border border-green-500/30"
                  : "bg-red-500/20 text-red-300 border border-red-500/30"
              }`}
            >
              <div
                className={`w-2 h-2 rounded-full mr-2 ${
                  vpnState.connected ? "bg-green-400" : "bg-red-400"
                }`}
              ></div>
              {vpnState.connected ? "Connected" : "Disconnected"}
            </div>
          </div>

          {/* Connection Form */}
          {!vpnState.connected && (
            <div className="space-y-4 mb-6">
              <div>
                <label className="block text-sm font-medium text-slate-300 mb-2">
                  Username
                </label>
                <input
                  type="text"
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  className="w-full px-3 py-2 bg-white/10 border border-white/20 rounded-lg text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-purple-500"
                  placeholder="Enter username"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-300 mb-2">
                  Server URL
                </label>
                <input
                  type="text"
                  value={serverUrl}
                  onChange={(e) => setServerUrl(e.target.value)}
                  className="w-full px-3 py-2 bg-white/10 border border-white/20 rounded-lg text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-purple-500"
                  placeholder="ws://localhost:8080/ws"
                />
              </div>
            </div>
          )}

          {/* Error Message */}
          {vpnState.error && (
            <div className="mb-4 p-3 bg-red-500/20 border border-red-500/30 rounded-lg text-red-300 text-sm">
              {vpnState.error}
            </div>
          )}

          {/* Connect/Disconnect Button */}
          <button
            onClick={vpnState.connected ? disconnect : connect}
            disabled={vpnState.connecting}
            className={`w-full py-3 px-4 rounded-lg font-medium transition-all ${
              vpnState.connected
                ? "bg-red-600 hover:bg-red-700 text-white"
                : "bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700 text-white"
            } disabled:opacity-50 disabled:cursor-not-allowed`}
          >
            {vpnState.connecting ? (
              <div className="flex items-center justify-center">
                <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white mr-2"></div>
                Connecting...
              </div>
            ) : vpnState.connected ? (
              "Disconnect"
            ) : (
              "Connect"
            )}
          </button>

          {/* Connection Stats */}
          {vpnState.connected && (
            <div className="mt-6 space-y-3">
              {/* Server Info */}
              {vpnState.serverInfo && (
                <div className="bg-white/5 rounded-lg p-3 mb-3">
                  <div className="text-xs text-slate-400 mb-1">
                    Connected Server
                  </div>
                  <div className="text-sm font-medium text-white">
                    {vpnState.serverInfo.name}
                  </div>
                  <div className="text-xs text-slate-400">
                    {vpnState.serverInfo.encryption}
                  </div>
                  <div className="text-xs text-green-400 mt-1">
                    {vpnState.serverInfo.ip_address}:{vpnState.serverInfo.port}
                  </div>
                </div>
              )}

              <div className="grid grid-cols-2 gap-4">
                <div className="bg-white/5 rounded-lg p-3 text-center">
                  <div className="text-xs text-slate-400 mb-1">Latency</div>
                  <div className="text-lg font-semibold text-white">
                    {vpnState.latency ? `${vpnState.latency}ms` : "--"}
                  </div>
                </div>
                <div className="bg-white/5 rounded-lg p-3 text-center">
                  <div className="text-xs text-slate-400 mb-1">Status</div>
                  <div className="text-lg font-semibold text-green-400">
                    Secure
                  </div>
                </div>
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div className="bg-white/5 rounded-lg p-3 text-center">
                  <div className="text-xs text-slate-400 mb-1">Downloaded</div>
                  <div className="text-sm font-medium text-white">
                    {formatBytes(vpnState.bytesTransferred.rx)}
                  </div>
                </div>
                <div className="bg-white/5 rounded-lg p-3 text-center">
                  <div className="text-xs text-slate-400 mb-1">Uploaded</div>
                  <div className="text-sm font-medium text-white">
                    {formatBytes(vpnState.bytesTransferred.tx)}
                  </div>
                </div>
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div className="bg-white/5 rounded-lg p-3 text-center">
                  <div className="text-xs text-slate-400 mb-1">Server Load</div>
                  <div className="text-sm font-medium text-white">
                    {vpnState.serverLoad}%
                  </div>
                </div>
                <div className="bg-white/5 rounded-lg p-3 text-center">
                  <div className="text-xs text-slate-400 mb-1">Users</div>
                  <div className="text-sm font-medium text-white">
                    {vpnState.connectedUsers}
                  </div>
                </div>
              </div>
            </div>
          )}

          {/* VPN Tunneling Test */}
          {vpnState.connected && (
            <div className="mt-6">
              <div className="flex items-center justify-between mb-3">
                <h3 className="text-sm font-medium text-slate-300">
                  Tunnel Test
                </h3>
                <button
                  onClick={clearLog}
                  className="text-xs text-slate-400 hover:text-white"
                >
                  Clear Log
                </button>
              </div>
              <div className="flex gap-2 mb-3">
                <input
                  type="text"
                  value={tunnelData}
                  onChange={(e) => setTunnelData(e.target.value)}
                  placeholder="Enter data to tunnel..."
                  className="flex-1 px-3 py-2 bg-white/10 border border-white/20 rounded-lg text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-purple-500 text-sm"
                />
                <button
                  onClick={() => {
                    if (tunnelData.trim()) {
                      sendTunnelData(tunnelData);
                      setTunnelData("");
                    }
                  }}
                  className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-medium"
                >
                  Send
                </button>
              </div>

              {/* Connection Log */}
              <div className="bg-black/20 rounded-lg p-3 max-h-32 overflow-y-auto">
                <div className="text-xs text-slate-400 mb-2">
                  Connection Log:
                </div>
                {connectionLog.length === 0 ? (
                  <div className="text-xs text-slate-500">
                    No activity yet...
                  </div>
                ) : (
                  connectionLog.slice(-5).map((log, index) => (
                    <div key={index} className="text-xs text-green-400 mb-1">
                      {new Date().toLocaleTimeString()}: {log}
                    </div>
                  ))
                )}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
