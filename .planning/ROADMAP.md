# Roadmap: CSGORoll Skin Sniper

## Overview

Six phases deliver the complete v1.0 trading engine. Phase 1 lays the Rust/Tokio scaffold and credential loading. Phase 2 establishes the persistent WebSocket connection. Phase 3 builds the in-memory whitelist engine — the core decision primitive. Phase 4 wires buy execution through a keep-alive HTTP connection with CapSolver challenge resolution. Phase 5 adds balance and inventory tracking with graceful-exit logic. Phase 6 adds async observability and latency measurement outside the hot path.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Scaffold** - Rust/Tokio project skeleton with env credential loading
- [ ] **Phase 2: WebSocket Connection** - Persistent, auto-reconnecting WebSocket feed
- [ ] **Phase 3: Whitelist Engine** - In-memory O(1) whitelist matching with zero disk I/O
- [ ] **Phase 4: Buy Execution** - Zero-latency keep-alive HTTP buy path with CapSolver integration — buy faster than any competitor
- [ ] **Phase 5: Balance and Inventory** - Coin balance tracking, per-item guards, graceful exit
- [ ] **Phase 6: Observability** - Async logging and end-to-end latency measurement

## Phase Details

### Phase 1: Scaffold
**Goal**: A runnable Rust binary exists with Tokio runtime, dependency graph, and all credentials loaded from environment at startup
**Depends on**: Nothing (first phase)
**Requirements**: CONN-02, PROT-03
**Success Criteria** (what must be TRUE):
  1. `cargo build` succeeds with Tokio, tokio-tungstenite, reqwest, ahash, dotenvy, capsolver-rs (or equivalent) in Cargo.toml
  2. Running the binary reads CSGOROLL_API_TOKEN, CSGOROLL_SESSION, and CAPSOLVER_API_KEY from environment or .env file and logs confirmation that all three are present
  3. Binary exits with a clear error message if any required credential is missing
**Plans**: TBD

Plans:
- [ ] 01-01: Initialize Cargo workspace, add dependencies, configure Tokio runtime entry point
- [ ] 01-02: Implement env/dotenv credential loading with startup validation

### Phase 2: WebSocket Connection
**Goal**: The bot connects to CSGORoll's WebSocket feed, receives live listing events, and automatically reconnects with exponential backoff after any disconnect
**Depends on**: Phase 1
**Requirements**: CONN-01
**Success Criteria** (what must be TRUE):
  1. Bot establishes authenticated WebSocket connection to CSGORoll and prints the first received listing event to stdout
  2. When the connection is forcibly dropped (e.g., network kill), the bot reconnects and resumes receiving events without process restart
  3. Reconnect delay follows exponential backoff (observable in logs: 1s, 2s, 4s, ... capped)
**Plans**: TBD

Plans:
- [ ] 02-01: Implement WebSocket connect loop with Tokio, auth handshake, and raw frame logging
- [ ] 02-02: Add exponential backoff reconnect logic and connection health monitoring

### Phase 3: Whitelist Engine
**Goal**: Incoming listing events are evaluated against a pre-loaded in-memory whitelist with sub-microsecond O(1) lookups, zero disk I/O on the hot path, and full support for all matching dimensions
**Depends on**: Phase 2
**Requirements**: MATCH-01, MATCH-02, MATCH-03, MATCH-04, MATCH-05
**Success Criteria** (what must be TRUE):
  1. Bot loads a whitelist.json at startup and populates an AHashMap keyed on (skin_name, wear_tier, is_stattrak) before the WebSocket loop begins
  2. A listing event for a whitelisted item at or below the configured markup threshold triggers a "MATCH" log line; a non-matching event produces no log output
  3. All five wear tiers (Battle-Scarred, Well-Worn, Field-Tested, Minimal Wear, Factory New) are parsed from feed events and matched correctly against whitelist entries
  4. StatTrak and non-StatTrak variants of the same skin are treated as distinct whitelist entries and matched independently
  5. A listing outside configured min/max price bounds is rejected even if name, wear, and markup match
**Plans**: TBD

Plans:
- [ ] 03-01: Define whitelist entry types, implement AHashMap loading from whitelist.json, add wear-tier and StatTrak parsing
- [ ] 03-02: Implement hot-path evaluation function: lookup → markup check → price bounds check → match/reject decision

### Phase 4: Buy Execution
**Goal**: On a whitelist match the bot fires an authenticated HTTP POST buy request in the minimum possible time — over a persistent keep-alive connection with no per-order TCP/TLS overhead — resolving any Cloudflare or captcha challenge via CapSolver to ensure the buy lands before any competitor
**Depends on**: Phase 3
**Requirements**: CONN-03, PROT-01, PROT-02, TRADE-01
**Success Criteria** (what must be TRUE):
  1. The bot issues a buy request within the same Tokio task as the match decision using a pre-established HTTP/1.1 keep-alive connection (no new TCP or TLS handshake per order, verifiable via network trace or timing log)
  2. When the buy endpoint responds with a Cloudflare challenge, the bot resolves it via CapSolver and retries the request without manual intervention
  3. When the buy endpoint responds with a captcha challenge, the bot resolves it via CapSolver and retries the request without manual intervention
  4. A successful buy response is logged; a failed buy response (after challenge resolution) is logged with status code and error body
**Plans**: TBD

Plans:
- [ ] 04-01: Implement persistent reqwest HTTP client with keep-alive, pre-authenticate connection, and wire TRADE-01 buy request on match
- [ ] 04-02: Implement CapSolver integration for Cloudflare and captcha challenge detection and resolution in the buy flow

### Phase 5: Balance and Inventory
**Goal**: The bot tracks coin balance and session inventory in memory, silently skips buys it cannot afford, and exits cleanly when funds fall below the configured floor
**Depends on**: Phase 4
**Requirements**: TRADE-02, TRADE-03, TRADE-04
**Success Criteria** (what must be TRUE):
  1. After each successful buy the in-memory coin balance decreases by the item price and the item is appended to the session inventory list
  2. When a matching item's price exceeds the current balance the bot logs a "SKIP insufficient balance" message and does not fire a buy request
  3. When the coin balance drops below the configured global minimum purchase price the bot logs "EXIT balance below minimum" and terminates the process with exit code 0
**Plans**: TBD

Plans:
- [ ] 05-01: Implement in-memory balance and inventory state, integrate balance deduction on buy, skip logic for insufficient funds, and graceful exit on minimum threshold

### Phase 6: Observability
**Goal**: Every significant event (win, miss, error) is durably logged asynchronously outside the hot path, and end-to-end latency from WS frame receipt to buy request dispatch is measured and recorded for every match
**Depends on**: Phase 5
**Requirements**: OBS-01, OBS-02
**Success Criteria** (what must be TRUE):
  1. Buy wins, whitelist misses, and errors appear in a persistent log file (or append-only sink) after the process exits; log writes never block the WebSocket receive loop
  2. Each match event produces a latency record (in microseconds) measuring the interval from WebSocket frame received to HTTP buy request dispatched, visible in the log output
  3. The hot path (WS receive → parse → lookup → HTTP dispatch) contains zero synchronous log writes or file I/O (verifiable by code inspection)
**Plans**: TBD

Plans:
- [ ] 06-01: Integrate async logging via tracing + tracing-appender (or equivalent) with a background writer, instrument buy wins, misses, and errors
- [ ] 06-02: Add latency measurement using Instant::now() at frame receipt, record delta at request dispatch, emit as structured log field

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Scaffold | 0/2 | Not started | - |
| 2. WebSocket Connection | 0/2 | Not started | - |
| 3. Whitelist Engine | 0/2 | Not started | - |
| 4. Buy Execution | 0/2 | Not started | - |
| 5. Balance and Inventory | 0/1 | Not started | - |
| 6. Observability | 0/2 | Not started | - |
