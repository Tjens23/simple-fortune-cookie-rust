# Fortune Cookie Frontend - Rust Version

This is the Rust implementation of the Fortune Cookie frontend, converted from the original Go version and designed to work with the Rust backend.

## Features

- **HTTP API Server** - Frontend web server on port 8080
- **Backend Integration** - Communicates with Rust backend on port 9000
- **Static File Serving** - Serves HTML, CSS, and JavaScript files
- **Template Rendering** - Uses Handlebars for server-side rendering
- **Health Check** - `/healthz` endpoint for monitoring
- **Memory Safe** - Rust's ownership system prevents data races and memory leaks
- **Async Performance** - Uses Tokio for high-performance async I/O

## API Endpoints

- `GET /healthz` - Health check endpoint
- `GET /api/random` - Get a random fortune from backend
- `GET /api/all` - Get all fortunes from backend (HTML rendered)
- `POST /api/add` - Add a new fortune to backend
- `GET /` - Serve static files (index.html, script.js, etc.)

## Environment Variables

- `BACKEND_DNS` - Backend server hostname (optional, defaults to localhost)
- `BACKEND_PORT` - Backend server port (optional, defaults to 9000)

## Running the Application

```bash
# Development mode
cargo run

# Release mode (optimized)
cargo build --release
./target/release/fortune-frontend
```

## Frontend Architecture

The frontend serves as a proxy between the web UI and the backend API:

1. **Static Files**: Serves the HTML, CSS, and JavaScript files
2. **API Proxy**: Forwards requests to the backend and processes responses
3. **Template Rendering**: Converts JSON responses to HTML using Handlebars
4. **Error Handling**: Graceful error handling for backend connectivity issues

## Dependencies

- **tokio** - Async runtime
- **warp** - Web framework
- **serde** - Serialization/deserialization
- **reqwest** - HTTP client for backend communication
- **handlebars** - Template engine
- **rand** - Random number generation

## Integration with Backend

The frontend communicates with the Rust backend using HTTP requests:
- Forwards user requests to appropriate backend endpoints
- Handles JSON serialization/deserialization
- Renders backend responses as HTML when needed
- Maintains session-less communication

## Conversion Notes

This Rust frontend maintains full compatibility with the original Go implementation while providing:
- Better memory safety through Rust's ownership system
- Improved error handling with Result types
- Zero-cost async/await for better performance
- Strong typing to prevent runtime errors
- Uses rustls-tls for better portability (no OpenSSL dependency)
