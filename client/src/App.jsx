import { useState } from 'react'
import axios from 'axios'

function App() {
  const [url, setUrl] = useState('')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  const [status, setStatus] = useState(null)
  const [serverIp, setServerIp] = useState('')
  const [copied, setCopied] = useState(false)

  const handleSubmit = async (e) => {
    e.preventDefault()
    if (!url) return

    setLoading(true)
    setError('')
    setHtmlContent('')
    setStatus(null)
    setServerIp('')
    setCopied(false)

    try {
      const response = await axios.post('http://51.20.108.220:8888/proxy', {
        url: url
      })
      setStatus(response.data.status)
      setServerIp(response.data.server_ip)
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to fetch the URL. Please check if the proxy server is running.')
    } finally {
      setLoading(false)
    }
  }

  // Remove handleCopy and copy button since raw HTML is not shown

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 flex flex-col">
      {/* Header */}
      <header className="bg-black/30 backdrop-blur-lg border-b border-white/10 p-8 shadow-lg">
        <div className="max-w-4xl mx-auto flex flex-col items-center">
          <h1 className="text-4xl md:text-5xl font-extrabold text-white mb-2 text-center tracking-tight drop-shadow-xl bg-gradient-to-r from-purple-400 via-blue-400 to-pink-400 bg-clip-text text-transparent animate-gradient-x">🦀 Just Proxy Client</h1>
          <p className="text-slate-300 text-center text-lg font-medium">Browse any public site through your Rust proxy server</p>
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-1 p-6 flex flex-col items-center justify-center">
        <div className="w-full max-w-3xl flex flex-col gap-8">
          {/* URL Input Form */}
          <form onSubmit={handleSubmit} className="bg-white/10 backdrop-blur-xl border border-white/20 rounded-2xl shadow-2xl p-8 flex flex-col gap-6">
            <label htmlFor="url" className="text-lg font-semibold text-white mb-2">Website URL</label>
            <div className="flex flex-col sm:flex-row gap-4">
              <input
                id="url"
                type="url"
                value={url}
                onChange={(e) => setUrl(e.target.value)}
                placeholder="https://example.com"
                className="flex-1 px-6 py-4 bg-white/10 border border-white/20 rounded-xl text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all duration-200 shadow-inner"
                required
                autoFocus
              />
              <button
                type="submit"
                disabled={loading}
                className="px-8 py-4 bg-gradient-to-r from-purple-600 to-blue-600 text-white font-semibold rounded-xl hover:from-purple-700 hover:to-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 shadow-lg hover:shadow-xl"
              >
                {loading ? (
                  <div className="flex items-center gap-2">
                    <div className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                    Loading...
                  </div>
                ) : (
                  'Go'
                )}
              </button>
            </div>
          </form>

          {/* Feedback States */}
          {loading && (
            <div className="text-center py-8">
              <div className="inline-flex items-center gap-3 bg-white/10 backdrop-blur-lg px-8 py-5 rounded-xl shadow-lg animate-pulse">
                <div className="w-6 h-6 border-2 border-purple-400/30 border-t-purple-400 rounded-full animate-spin"></div>
                <span className="text-white text-lg font-medium">Fetching content through proxy...</span>
              </div>
            </div>
          )}

          {error && (
            <div className="bg-red-500/20 backdrop-blur-lg border border-red-500/30 text-red-200 px-8 py-5 rounded-xl shadow-lg animate-shake">
              <strong className="text-red-300">Error:</strong> {error}
            </div>
          )}

          {status && !loading && !error && (
            <div className="bg-green-500/20 backdrop-blur-lg border border-green-500/30 text-green-200 px-8 py-5 rounded-xl shadow-lg">
              <strong className="text-green-300">Status:</strong> {status}
            </div>
          )}

          {serverIp && !loading && !error && (
            <div className="bg-blue-500/20 backdrop-blur-lg border border-blue-500/30 text-blue-200 px-8 py-5 rounded-xl shadow-lg">
              <strong className="text-blue-300">Server IP:</strong> {serverIp}
            </div>
          )}

          {/* Content Display */}
          {htmlContent && !loading && !error && (
            <div className="flex flex-col gap-2 bg-white/5 backdrop-blur-2xl rounded-2xl border border-white/10 overflow-hidden shadow-2xl">
              <div className="flex items-center justify-between bg-white/10 px-6 py-3 border-b border-white/10">
                <h3 className="text-white font-medium">Proxied Content</h3>
              </div>
              <div className="p-0 sm:p-6 h-full min-h-[400px]">
                <iframe
                  srcDoc={htmlContent}
                  className="w-full h-[60vh] min-h-[400px] bg-white rounded-lg border-0 shadow-inner"
                  sandbox="allow-scripts allow-same-origin allow-forms allow-popups"
                  title="Proxied content"
                />
              </div>
            </div>
          )}

          {/* Empty State */}
          {!htmlContent && !loading && !error && (
            <div className="flex-1 flex items-center justify-center">
              <div className="text-center text-slate-400">
                <div className="text-6xl mb-4">🌐</div>
                <h3 className="text-xl font-semibold mb-2">Ready to Browse</h3>
                <p className="text-slate-500">Enter a URL above to start browsing through the proxy</p>
              </div>
            </div>
          )}
        </div>
      </main>
    </div>
  )
}

export default App
