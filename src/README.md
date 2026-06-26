# TrendAnalyzes Services (src)

Run the two backend services locally:

- `api-gateway`
- `analyze-api`

## Prerequisites

- Rust toolchain installed (`rustup`, `cargo`)
- Two terminals (one per service)

## Quick Start

### 1. Start `api-gateway`

```powershell
cd C:\Users\rodohern\projects\TrendAnalyzes\src\api-gateway
cargo build
cargo run
```

### 2. Start `analyze-api` (in another terminal)

```powershell
cd C:\Users\rodohern\projects\TrendAnalyzes\src\analyze-api
cargo build
cargo run
```

## API Docs

After both services are running, open:

- http://localhost:3000/swagger


