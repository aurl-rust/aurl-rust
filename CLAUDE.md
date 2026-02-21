# CLAUDE.md

This file provides guidance for Claude Code when working on the aurl-rust repository.

## Project Overview

**aurl-rust** is an OAuth2-enabled HTTP client CLI tool written in Rust. It automatically handles OAuth2 authentication flows (Authorization Code with PKCE, Password, Client Credentials), manages access token caching, and makes HTTP requests with automatic token injection. It can also output curl command snippets instead of executing requests directly.

## Build and Test Commands

```bash
# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run all tests
cargo test --verbose

# Format code (required before committing)
cargo fmt

# Lint (warnings are treated as errors in CI)
cargo clippy -- -D warnings

# Run both fmt check and clippy (mirrors CI)
cargo fmt && cargo clippy -- -D warnings && cargo test --verbose
```

## Architecture

```
src/
├── main.rs           # Entry point; initializes logger and calls CLI execution
├── cli.rs            # Main CLI logic; loads profiles, creates HTTP client, runs dispatcher
├── options.rs        # Parses command-line arguments via clap
├── profile.rs        # Reads ~/.aurl/profiles (INI format) for OAuth2 config
├── oauth2.rs         # Core OAuth2 implementation; all three grant types; token caching
├── authserver.rs     # Embedded HTTP server on port 8080 to receive OAuth2 callback
├── logger.rs         # Configures log4rs with debug/info verbosity levels
├── version.rs        # Version info
├── output.rs         # Generates curl command snippets
└── request/
    ├── mod.rs
    ├── dispatcher.rs      # Orchestrates token retrieval, request building, and sending
    ├── response.rs        # Response types
    ├── error.rs           # Request error types
    ├── cors.rs            # CORS/redirect policy (same-origin enforcement)
    └── modifier/          # Composable request pipeline
        ├── mod.rs
        ├── auth_header.rs     # Authorization header injection
        ├── custom_headers.rs  # Custom headers (-H flags)
        ├── body.rs            # Request body (-d flag)
        ├── headers.rs         # General headers
        └── timeout.rs         # Request timeout
```

**Key design patterns:**
- Tokio async runtime for all I/O
- Request modifiers implement a trait pattern for composable request building
- OAuth2 token caching in `~/.aurl/token/<profile>.json` with TTL-based expiration
- Expired tokens are automatically refreshed; 401 responses invalidate the cache

## Runtime Configuration

OAuth2 profiles are stored in `~/.aurl/profiles` (INI format):

```ini
[my-profile]
default_content_type = application/json
grant_type = authorization_code
client_id = YOUR_CLIENT_ID
client_secret = YOUR_CLIENT_SECRET
auth_server_token_endpoint = https://example.auth0.com/oauth/token
auth_server_auth_endpoint = https://example.auth0.com/authorize
scopes = openid profile
redirect = http://localhost:8080/callback
# Optional: custom auth header instead of default "Authorization: Bearer <token>"
default_auth_header_template = x-custom-auth=$token
```

Supported `grant_type` values: `authorization_code` (or `auth`), `password`, `client_credentials` (or `client`).

## CI/CD

GitHub Actions runs on every push (`.github/workflows/test.yml`):
1. `cargo fmt` — format check
2. `cargo clippy -- -D warnings` — lint with warnings as errors
3. `cargo test --verbose` — run all tests

Targets tested: `x86_64-apple-darwin`, `aarch64-apple-darwin`, `x86_64-pc-windows-gnu`.

Release workflow (`.github/workflows/release.yml`) triggers on main branch pushes and creates draft GitHub releases with macOS binaries.

## Rust Version

The project specifies Rust `1.60.0` in `rust-toolchain`. Ensure the installed toolchain matches.
