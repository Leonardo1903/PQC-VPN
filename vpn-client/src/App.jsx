import { useState } from 'react'
import axios from 'axios'

function App() {
  const [url, setUrl] = useState('')
  const [htmlContent, setHtmlContent] = useState('')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  const [status, setStatus] = useState(null)

  const handleSubmit = async (e) => {
    e.preventDefault()
    if (!url) return

    setLoading(true)
    setError('')
    setHtmlContent('')
    setStatus(null)

    try {
      const response = await axios.post('http://localhost:8888/proxy', {
        url: url
      })

      setHtmlContent(response.data.html)
      setStatus(response.data.status)
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to fetch the URL. Please check if the proxy server is running.')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 flex flex-col">
      {/* Header */}
      <header className="bg-black/20 backdrop-blur-sm border-b border-white/10 p-6">
        <div className="max-w-7xl mx-auto">
          <h1 className="text-4xl md:text-5xl font-bold text-white mb-2 text-center">
            🦀 Simple Proxy Client
          </h1>
          <p className="text-slate-300 text-center text-lg">
            Enter any URL to browse through the proxy server
          </p>
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-1 p-6">
        <div className="max-w-7xl mx-auto h-full flex flex-col">
          
          {/* URL Input Form */}
          <div className="mb-8">
            <form onSubmit={handleSubmit} className="max-w-4xl mx-auto">
              <div className="flex flex-col sm:flex-row gap-4">
                <input
                  type="url"
                  value={url}
                  onChange={(e) => setUrl(e.target.value)}
                  placeholder="https://example.com"
                  className="flex-1 px-6 py-4 bg-white/10 backdrop-blur-sm border border-white/20 rounded-xl text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all duration-200"
                  required
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
          </div>

          {/* Loading State */}
          {loading && (
            <div className="text-center py-12">
              <div className="inline-flex items-center gap-3 bg-white/10 backdrop-blur-sm px-6 py-4 rounded-xl">
                <div className="w-6 h-6 border-2 border-purple-400/30 border-t-purple-400 rounded-full animate-spin"></div>
                <span className="text-white text-lg">Fetching content through proxy...</span>
              </div>
            </div>
          )}

          {/* Error Messages */}
          {error && (
            <div className="bg-red-500/20 backdrop-blur-sm border border-red-500/30 text-red-200 px-6 py-4 rounded-xl mb-6">
              <strong className="text-red-300">Error:</strong> {error}
            </div>
          )}

          {/* Status Messages */}
          {status && (
            <div className="bg-green-500/20 backdrop-blur-sm border border-green-500/30 text-green-200 px-6 py-4 rounded-xl mb-6">
              <strong className="text-green-300">Status:</strong> {status}
            </div>
          )}

          {/* Content Display */}
          {htmlContent && (
            <div className="flex-1 bg-white/5 backdrop-blur-sm rounded-xl border border-white/10 overflow-hidden shadow-2xl">
              <div className="bg-white/10 px-6 py-3 border-b border-white/10">
                <h3 className="text-white font-medium">Proxied Content</h3>
              </div>
              <div className="p-6 h-full">
                <iframe
                  srcDoc={htmlContent}
                  className="w-full h-full min-h-[600px] bg-white rounded-lg border-0 shadow-inner"
                  sandbox="allow-scripts allow-same-origin allow-forms allow-popups"
                  title="Proxied content"
                />
              </div>
            </div>
          )}

          {/* Empty State */}
          {!htmlContent && !loading && (
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
