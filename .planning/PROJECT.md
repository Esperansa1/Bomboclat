# CSGORoll Skin Sniper

## What This Is

An ultra-low-latency automated trading engine that monitors CSGORoll's real-time listing feed via WebSocket, detects items from a pre-configured whitelist posted at 0% (or minimal) markup, and fires a purchase request faster than competing buyers. Built in Rust for deterministic, maximum-speed execution. Operates within CSGORoll's Terms of Service on a single account.

## Core Value

React to a new whitelisted listing before any other buyer does — the engine wins when it snipes items that would otherwise be taken by competitors within milliseconds.

## Current Milestone: v1.0 — Trading Engine

**Goal:** Build the complete Rust trading engine — from WebSocket connection to purchase execution — with balance tracking, configurable whitelist, and observability.

**Target features:**
- Persistent WebSocket connection with auto-reconnect
- In-memory whitelist lookup (name × wear × StatTrak) with per-item markup + price thresholds
- Buy execution with pre-authenticated keep-alive HTTP connection
- Balance and inventory tracking; graceful exit on low funds
- Async logging outside the hot path
- End-to-end latency measurement

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] Connect to CSGORoll's WebSocket feed and maintain a persistent, auto-reconnecting connection
- [ ] Ingest listing events in real time with minimal parsing overhead
- [ ] Pre-load item whitelist from JSON config (wear × skin name × StatTrak flag) into an in-memory O(1) lookup structure at startup
- [ ] Evaluate each incoming listing against whitelist and markup threshold with zero disk I/O
- [ ] Fire buy API request immediately on match using a pre-authenticated, keep-alive HTTP connection
- [ ] Support per-item markup threshold configuration (default: 0%, configurable per skin/wear/StatTrak)
- [ ] Support per-item min/max price configuration; global min price triggers process exit when balance falls below it
- [ ] Differentiate StatTrak and non-StatTrak items in whitelist matching
- [ ] Support all five wear tiers: Battle-Scarred, Well-Worn, Field-Tested, Minimal Wear, Factory New
- [ ] Track coin balance and session inventory (list of items bought) in memory
- [ ] Skip buy attempt silently if balance is insufficient for a specific item's price
- [ ] Exit process cleanly when balance drops below configured global minimum purchase price
- [ ] Log wins, misses, and errors to a persistent store outside the hot path
- [ ] Load auth credentials (token/session) from environment variables or .env file at startup
- [ ] Deploy on a VPS geographically co-located with CSGORoll's servers
- [ ] Measure and expose end-to-end latency (WS event received → buy request sent) for tuning

### Out of Scope

- Multi-platform support (DMarket, Skinport, Steam) — CSGORoll only for v1
- Multi-account operation — single account only
- External price validation (Steam median, CSGOFloat) — markup from CSGORoll feed is the sole decision signal
- Portfolio management / P&L tracking — out of scope for trading engine
- Web UI or dashboard — config via files, monitoring via logs
- Mobile alerts or Telegram/Discord notifications — v2 if needed

## Context

- CSGORoll exposes markup percentage directly in listing events — no external price oracle needed. The decision is: `item in whitelist AND markup <= threshold`.
- Auth tokens/API credentials for CSGORoll are already obtained. The buy endpoint is known.
- Competitors are likely running similar bots. The race is won or lost in sub-10ms windows.
- The hot path is: WS frame received → parse → hash lookup → threshold check → HTTP POST. Every microsecond shaved here is competitive advantage.
- CSGORoll is confirmed TOS-compliant for this use case.

## Constraints

- **Language**: Rust — non-negotiable, chosen for deterministic latency and zero-cost abstractions
- **Runtime**: Tokio async runtime for non-blocking I/O across WS + HTTP
- **Hot path**: Zero disk I/O, zero heap allocation where avoidable, no locks on the critical decision path
- **Infra**: Single VPS, location TBD based on CSGORoll datacenter proximity (likely EU)
- **Concurrency**: Lock-free or minimally-locked data structures for the whitelist lookup (read-heavy, write-rare)
- **Network**: Persistent WebSocket connection with exponential backoff reconnect; persistent HTTP/1.1 keep-alive for buy requests (avoid TLS handshake on every order)

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust over Go | Deterministic performance, no GC pauses, zero-cost abstractions — critical for sub-ms latency | — Pending |
| Tokio async runtime | Best-in-class async I/O for Rust, mature WS + HTTP ecosystem | — Pending |
| In-memory AHashMap for whitelist | AHashMap uses non-cryptographic hashing (AHash) — faster than std HashMap for hot lookups | — Pending |
| CSGORoll markup as sole decision signal | Eliminates external API latency from hot path entirely | — Pending |
| Single persistent HTTP connection for buys | Avoids TCP + TLS setup cost on every purchase attempt | — Pending |

---
*Last updated: 2026-02-24 after milestone v1.0 definition*
