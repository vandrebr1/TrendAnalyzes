# Trend Analyzes

> Engine that collects and analyzes news/social data to detect trends using heuristics and statistics. It identifies spikes, patterns, and emerging topics, enabling fast, transparent insights.

## Overview

Trend Analyzes is a data-driven trend analysis system designed to continuously
collect, normalize, and process data from multiple distributed sources (news
feeds, forums, and selected social platforms) in order to detect emerging
topics, patterns, and unusual activity in near real-time.

Instead of relying on opaque, resource-intensive machine learning models, the
system uses a **lightweight, transparent, heuristic- and statistics-driven**
approach. It extracts meaningful signals from raw text through:

- Content normalization
- Keyword extraction
- Frequency tracking over time
- Co-occurrence analysis
- **Burst detection** — significant deviations in term usage over time
- Rule-based thresholds to highlight emerging discussions

By combining multiple independent sources, the system reduces bias and increases
the reliability of detected trends — while staying **cost-effective,
explainable, and maintainable**.

## Tech Stack

| Concern              | Choice                          |
| -------------------- | ------------------------------- |
| Language             | Rust (edition 2024)             |
| Async runtime        | [Tokio](https://tokio.rs)       |
| HTTP client          | [Reqwest](https://docs.rs/reqwest) (rustls) |
| Web framework / API  | [Axum](https://docs.rs/axum)    |
| Storage              | SQLite via [rusqlite](https://docs.rs/rusqlite) (bundled) |
| Serialization        | Serde / serde_json              |
| Observability        | tracing / tracing-subscriber    |

## Requirements

- [Rust](https://rustup.rs) 1.96+ (edition 2024)
- A C toolchain (provided automatically on Windows via the MSVC build tools;
  required by the bundled SQLite build)

## Getting Started

```bash
# Build
cargo build

# Run (starts the HTTP server on http://127.0.0.1:3000)
cargo run
```

Then, in another terminal:

```bash
curl http://127.0.0.1:3000/
# -> Hello, Trend Analyzes!

curl http://127.0.0.1:3000/health
# -> {"status":"ok","service":"trend_analyzes","version":"0.1.0"}
```

### Logging

Log verbosity is controlled by the `RUST_LOG` environment variable:

```bash
# Windows (PowerShell)
$env:RUST_LOG="debug"; cargo run

# Linux / macOS
RUST_LOG=debug cargo run
```

## Project Status

🚧 **Early scaffolding.** This is the initial "hello world" skeleton: an Axum
server with a root endpoint and a `/health` check. The ingestion, processing,
and analysis pipeline described in [ARCHITECTURE.md](ARCHITECTURE.md) is not yet
implemented.

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) — system design, modules, and data flow.

## License

MIT
