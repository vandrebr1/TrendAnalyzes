Trend Analyzer (Rust) — Microservices Architecture

## Overview

This document describes the intended microservices architecture for the
"Trend Analyzer" project. It outlines component responsibilities, data flows,
and integration points to guide implementation and operations.

---

## 1. API Gateway (Controller)

Purpose
- Acts as the system entry point for client traffic.
- Routes requests to the Analyze API or Trend API.
- Performs authentication, authorization, validation, and request logging.

Responsibilities
- Accept HTTP requests from front-end clients.
- Forward validated requests to internal services and return their responses.
- Do not implement business logic; only request coordination.

## 2. Analyze API

Purpose
- Provide an on-demand analysis interface for clients.

Responsibilities
- Validate incoming request payloads.
- Orchestrate calls to the Analyzer Service.
- Format and return results to the client.

Input/Output
- Receives JSON requests with analysis parameters.
- Returns structured analysis responses.

## 3. Analyzer Service (Core)

Purpose
- Implement the core processing pipeline for trend detection.

Responsibilities
- Coordinate data retrieval and collection.
- Execute text processing, metric computation, and heuristic-based trend detection.
- Return raw/structured results for the API to format.

Processing flow
1. Data validation: query the database and decide if additional data is required.
2. Data collection (if needed): invoke the Collector Service.
3. Data retrieval: read stored and newly collected data.
4. Filtering: select items that match configured keywords or criteria.
5. Text processing: normalize and tokenize content.
6. Metric computation: compute mentions, growth rates, and statistical scores.
7. Trend detection: apply heuristic rules to identify candidate trends.

Notes
- The Analyzer Service produces internal raw results; the Analyze API is
  responsible for shaping the final response returned to clients.

## 4. Trend API

Purpose
- Serve precomputed trends and associated metadata.

Responsibilities
- Query the storage layer for computed trends.
- Apply filtering and sorting according to client parameters.
- Return the final list of trends and metadata.

## 5. Storage (Database)

Purpose
- Persist raw inputs, processed artifacts, and computed results.

Responsibilities
- Store collected posts, tokens, time-series term counts, trends, and job
  configurations.
- Serve data to the Analyzer Service, Trend API, and Scheduler.

Example schema components
- raw_posts (id, source, url, fetched_at, content)
- tokens (post_id, token, position)
- term_frequency (term_id, bucket_start, count)
- trends (id, term_id, score, detected_at, window)
- analysis_jobs (id, keywords, schedule, last_run)

## 6. Collector Service

Purpose
- Fetch external content from sources such as RSS feeds and social platforms.

Responsibilities
- Execute scrapers and feed collectors (RSS, Reddit, etc.).
- Parse and optionally pre-filter content by keywords.
- Persist collected data to the database and return data to the Analyzer
  Service when requested.

## 7. Scheduler

Purpose
- Manage and trigger periodic analysis jobs.

Responsibilities
- Read configured jobs from the database (keywords, time range, interval).
- Execute jobs on the configured schedule and invoke the Analyzer Service.

## Overall flow

Scheduler → Analyzer Service → Collector Service → Database → Analyzer Service → Database

---

If you want, I can now: commit this change, add a table-of-contents, or convert
this to a more formal markdown layout with badges and links.

