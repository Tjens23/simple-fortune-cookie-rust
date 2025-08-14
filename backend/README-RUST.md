# Fortune Cookie Backend - Rust Version

This is the Rust implementation of the Fortune Cookie backend, converted from the original Go version.

## Features

- **HTTP API Server** - RESTful API for fortune management
- **Redis Integration** - Optional Redis support for persistent storage
- **Memory-Safe** - Rust's ownership system prevents data races and memory leaks
- **Async Performance** - Uses Tokio for high-performance async I/O
- **Thread-Safe** - Concurrent access to fortune store using Arc<RwLock>

## API Endpoints

- `GET /fortunes` - List all fortunes
- `GET /fortunes/{id}` - Get a specific fortune by ID
- `GET /fortunes/random` - Get a random fortune
- `POST /fortunes` - Create a new fortune

## Environment Variables

- `REDIS_DNS` - Redis server hostname (optional, defaults to localhost)

## Running the Application

```bash
# Development mode
cargo run

# Release mode (optimized)
cargo build --release
./target/release/fortune-backend
```

## Default Fortunes

The application comes with 4 default fortunes:
1. "A new voyage will fill your life with untold memories."
2. "The measure of time to your next goal is the measure of your discipline."
3. "The only way to do well is to do better each day."
4. "It ain't over till it's EOF."

## Redis Support

If the `REDIS_DNS` environment variable is set, the application will:
- Connect to Redis on port 6379
- Load existing fortunes from the "fortunes" hash
- Persist new fortunes to Redis
- Fall back gracefully if Redis is unavailable

## Dependencies

- **tokio** - Async runtime
- **warp** - Web framework
- **serde** - Serialization/deserialization
- **redis** - Redis client
- **rand** - Random number generation

## Conversion Notes

This Rust version maintains full API compatibility with the original Go implementation while providing:
- Better memory safety through Rust's ownership system
- Improved error handling with Result types
- Zero-cost async/await for better performance
- Strong typing to prevent runtime errors
