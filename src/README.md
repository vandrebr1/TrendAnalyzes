# TrendAnalyzes Services (src)

Run the two backend services locally:

- `api-gateway`
- `analyzer_service`
- `ai_service`

## Prerequisites

- Rust toolchain installed (`rustup`, `cargo`)
- Two terminals (one per service)

## Quick Start

### 1. Start `api-gateway`

```powershell
cd C:\..\TrendAnalyzes\src\api-gateway
cargo build
cargo run
```

### 2. Start `analyzer-service` (in another terminal)

```powershell
cd C:\..\TrendAnalyzes\src\analyzer-service
cargo build
cargo run
```

### 3. Start `ai_service` (in another terminal)

```powershell
cd C:\..\TrendAnalyzes\src\ai_service
# copy .env.example to .env and update CAPGEN_API_KEY
# ai_service loads .env automatically on startup
cargo build
cargo run
```

## API Docs

After both services are running, open:

- http://localhost:3000/swagger
- http://localhost:3050/swagger
- http://localhost:3060/swagger


