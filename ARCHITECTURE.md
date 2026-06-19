# Architecture

> This document describes the intended design of Trend Analyzes. Most of it is
> **not yet implemented** — the current codebase is a minimal Axum skeleton.
> Treat this as the roadmap that the modules should grow into.

## Goals

- **Transparent & explainable** — every detected trend traces back to concrete
  term frequencies and rules, not an opaque model.
- **Modular** — ingestion, processing, analysis, storage, and the API are
  independent components with clear boundaries.
- **Scalable** — continuous ingestion and asynchronous processing built on Tokio.
- **Cost-effective** — heuristics and statistics instead of external ML services.

## High-Level Data Flow

```
                +-------------+      +--------------+      +------------------+
  Sources  ---> |  Ingestion  | ---> |  Processing  | ---> |     Analysis     | ---> Trends
 (news,         | (collectors)|      | (normalize,  |      | (burst detection,|
  forums,       +------+------+      |  tokenize,   |      |  co-occurrence,  |
  social)              |             |  keyword     |      |  thresholds)     |
                       |             |  extraction) |      +--------+---------+
                       v             +------+-------+               |
                  +---------+               |                       v
                  | Storage | <-------------+----------------> +---------+
                  | (SQLite)|                                  |   API   | (Axum)
                  +---------+                                  +---------+
```

## Planned Modules

| Module        | Responsibility                                                        |
| ------------- | --------------------------------------------------------------------- |
| `ingestion`   | Source collectors (HTTP via Reqwest), scheduling, rate limiting, dedup. |
| `processing`  | Normalization, tokenization, keyword extraction, frequency counting.  |
| `analysis`    | Burst detection, co-occurrence, rule-based thresholds, scoring.       |
| `storage`     | SQLite access layer (rusqlite): documents, terms, time-series counts. |
| `api`         | Axum HTTP layer exposing trends, health, and query endpoints.         |
| `model`       | Shared domain types (Document, Term, Trend, TimeBucket).              |

## Concurrency Model

- A multi-threaded Tokio runtime drives the whole process.
- Ingestion collectors run as long-lived async tasks, one per source, feeding a
  shared channel (`tokio::sync::mpsc`).
- Processing consumes from that channel, transforms documents, and persists
  term counts into SQLite.
- Because rusqlite is **synchronous/blocking**, all DB access should run inside
  `tokio::task::spawn_blocking` (or a dedicated blocking worker / connection
  pool) to avoid stalling the async runtime.
- The Axum API reads aggregated results from storage to serve trend queries.

## Trend Detection (Heuristics)

1. **Time bucketing** — group term occurrences into fixed windows (e.g. hourly).
2. **Frequency tracking** — maintain per-term counts per window.
3. **Burst detection** — flag a term when its current-window frequency deviates
   significantly from its recent baseline (e.g. a z-score or ratio threshold).
4. **Co-occurrence** — track term pairs that spike together to surface topics
   rather than isolated keywords.
5. **Thresholds & scoring** — rule-based gates decide what qualifies as an
   "emerging trend" and rank results.

## Storage Sketch (SQLite)

Indicative tables (to be refined during implementation):

- `documents(id, source, url, fetched_at, content)`
- `terms(id, term)`
- `term_counts(term_id, bucket_start, count)` — the time series that powers burst detection
- `cooccurrences(term_a, term_b, bucket_start, count)`
- `trends(id, term_id, score, detected_at, window)`

## Current vs. Planned

| Component            | Status            |
| -------------------- | ----------------- |
| Axum server + health | ✅ Implemented     |
| Ingestion            | ⬜ Planned         |
| Processing           | ⬜ Planned         |
| Analysis             | ⬜ Planned         |
| Storage layer        | ⬜ Planned         |
