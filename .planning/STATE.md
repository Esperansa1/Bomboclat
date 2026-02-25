# State: CSGORoll Skin Sniper

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-24)

**Core value:** React to a new whitelisted listing before any other buyer does — the engine wins when it snipes items that would otherwise be taken by competitors within milliseconds.
**Current focus:** Phase 2 - WebSocket Connection

## Current Position

Phase: 2 of 6 (WebSocket Connection)
Plan: 1 of 2 in current phase
Status: Plan 02-01 complete
Last activity: 2026-02-25 — Plan 02-01 complete: WebSocket connection module with Trade struct, connect_once, graphql-transport-ws handshake

Progress: [███░░░░░░░] 25%

## Performance Metrics

**Velocity:**
- Total plans completed: 3
- Average duration: 6.3 min
- Total execution time: 0.35 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-scaffold | 2 | 15 min | 7.5 min |
| 02-websocket-connection | 1 | 4 min | 4 min |

*Updated after each plan completion*

## Accumulated Context

### Decisions

- Rust + Tokio: non-negotiable for deterministic sub-ms latency
- AHashMap for whitelist: non-cryptographic hashing is faster on the read-heavy hot path
- Single persistent HTTP keep-alive connection: avoids TCP/TLS setup cost per buy
- CapSolver only on HTTP buy endpoint: WebSocket does not require challenge resolution
- CSGORoll markup as sole decision signal: eliminates external API latency from hot path
- Hot path constraint: zero disk I/O, zero heap allocation where avoidable, no locks on critical decision path
- capsolver crate TBD: neither capsolver nor capsolver-rs resolve on crates.io; Phase 4 will use reqwest directly against CapSolver REST API
- native-tls chosen over rustls for TLS on Windows (avoids OpenSSL packaging issues)
- [Phase 01-scaffold]: Used _config binding (not let _ = ...) to retain Config value in scope for Phase 2 WebSocket use
- [Phase 01-scaffold]: dead_code warning on Config fields not suppressed - will resolve naturally in Phase 2 when fields are consumed
- [Phase 02-websocket-connection]: futures-util and native-tls must be declared as direct Cargo.toml deps (Rust 2021: transitive deps not implicitly importable)
- [Phase 02-websocket-connection]: connect_once uses FnMut callback (not channel) to keep hot path allocation-free; Phase 3 can wrap in channel if needed

### Pending Todos

None yet.

### Blockers/Concerns

None yet.

## Session Continuity

Last session: 2026-02-25
Stopped at: Completed 02-websocket-connection-01-PLAN.md (WebSocket connection module with Trade struct and connect_once)
Resume file: None
