---
phase: 02-websocket-connection
plan: 01
subsystem: infra
tags: [rust, websocket, tokio-tungstenite, graphql-transport-ws, serde, serde_json, native-tls, futures-util]

# Dependency graph
requires:
  - phase: 01-scaffold
    provides: "Compilable Cargo.toml with tokio-tungstenite (native-tls), dotenvy, reqwest; src/config.rs Config struct with csgoroll_api_token and csgoroll_session fields"
provides:
  - "src/ws.rs: Trade struct with 9 fields matching Phase 3 interface (id, markup_percent, total_value, can_join, status, market_name, brand, skin_name, wear, is_stattrak)"
  - "src/ws.rs: pub async fn connect_once(config: &Config, on_trade: impl FnMut(Trade)) that completes graphql-transport-ws handshake and receives trade events"
  - "Cargo.toml: serde, serde_json, futures-util, native-tls added as explicit direct dependencies"
  - "src/main.rs: drives a single WebSocket connection attempt and prints each trade event to stdout"
affects: [03-trade-filter, 04-http-buy, 05-snipe-engine]

# Tech tracking
tech-stack:
  added:
    - "serde v1.0.228 (derive feature) - struct deserialization of WS message payloads"
    - "serde_json v1.0.149 - JSON parsing and construction for graphql-transport-ws messages"
    - "futures-util v0.3.32 - SinkExt/StreamExt traits for WebSocket send/next operations"
    - "native-tls v0.2.18 - explicit direct dep required to construct Connector::NativeTls"
  patterns:
    - "graphql-transport-ws handshake: connection_init with token payload, then connection_ack, then subscribe"
    - "WsMessage internal type for envelope; Trade public type for domain data - separate deserialization layers"
    - "parse_trade returns Option<Trade>: silently skips any event with missing fields (never panics on unexpected server data)"
    - "connect_once callback pattern: on_trade: impl FnMut(Trade) chosen for Phase 3 compatibility (no channel overhead)"
    - "StatTrak detection via market_name.starts_with('StatTrak™') prefix check"

key-files:
  created:
    - "src/ws.rs - WebSocket connection module: Trade struct, WsMessage, connect_once, parse_trade"
  modified:
    - "Cargo.toml - Added serde, serde_json, futures-util, native-tls"
    - "src/main.rs - Added mod ws; renamed _config to config; calls ws::connect_once with trade printer closure"

key-decisions:
  - "futures-util and native-tls must be declared as direct Cargo.toml deps (Rust 2021 edition: transitive deps from tokio-tungstenite are not implicitly importable)"
  - "connect_once uses callback (FnMut) not channel: avoids mpsc overhead on hot path; Phase 3 snipe engine can wrap in channel if needed"
  - "userId: null in subscription variables: omits per-user filter, receives all public trade listings"
  - "Box::new(e) instead of e.into() for tungstenite::Error in ack-wait loop: required for type inference to succeed"

patterns-established:
  - "WS module pattern: single connect_once function with callback, no internal state — reconnect logic is caller's responsibility (02-02)"
  - "Internal/public type separation: WsMessage (Deserialize) internal, Trade (Debug, Clone) public API"

requirements-completed: [CONN-01]

# Metrics
duration: 4min
completed: 2026-02-25
---

# Phase 2 Plan 01: WebSocket connection module with graphql-transport-ws handshake, Trade struct deserialization, and live trade event printing to stdout

**Authenticated WebSocket connection to wss://router.csgoroll.com/ws using graphql-transport-ws protocol; connection_init/connection_ack handshake; OnCreateTrades subscription; Trade struct with all 9 fields Phase 3 needs deserialized from server events**

## Performance

- **Duration:** 4 min
- **Started:** 2026-02-25T05:37:29Z
- **Completed:** 2026-02-25T05:41:23Z
- **Tasks:** 2
- **Files modified:** 3 (created: src/ws.rs; modified: Cargo.toml, src/main.rs)

## Accomplishments
- src/ws.rs created with `pub struct Trade` holding all 9 fields Phase 3 needs (id, markup_percent, total_value, can_join, status, market_name, brand, skin_name, wear, is_stattrak)
- `pub async fn connect_once` implements complete graphql-transport-ws handshake: connection_init with API token, connection_ack wait, OnCreateTrades subscribe, event receive loop
- `parse_trade` safely maps tradeItems[0].itemVariant fields; StatTrak detection from market_name prefix; returns None on any missing field (never panics)
- src/main.rs wired to call connect_once with closure that prints each trade's key fields
- cargo build exits 0 with zero errors; 2 expected dead_code warnings for future-phase fields (brand/skin_name/wear used in Phase 3; capsolver_api_key used in Phase 4)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add serde deps and implement ws.rs with Trade struct and connect_once** - `844d74a` (feat)
2. **Task 2: Wire connect_once into main.rs; fix missing direct deps** - `b24d5fc` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified
- `src/ws.rs` - WebSocket connection module: `pub struct Trade` (9 fields), `WsMessage` internal deserialize type, `pub async fn connect_once`, `fn parse_trade`
- `Cargo.toml` - Added serde v1 (derive), serde_json v1, futures-util v0.3, native-tls v0.2
- `src/main.rs` - Added `mod ws;`; renamed `_config` to `config`; calls `ws::connect_once` with trade-printing closure

## Decisions Made

- **futures-util and native-tls as explicit deps:** Rust 2021 edition does not allow importing from transitive dependencies. Both are transitive via tokio-tungstenite but must be declared in Cargo.toml to use directly in src/ws.rs. Added during Task 2 build as Rule 1 bug fix.
- **connect_once uses FnMut callback not channel:** Keeps the hot path allocation-free. Phase 3 snipe engine can wrap in a tokio channel if multi-consumer fanout is needed; keeping it simple here.
- **userId: null in subscription:** Omits per-user filter, receives all public trade listings from the feed — matches the sniping use case.
- **Box::new(e) for tungstenite error:** Type inference cannot resolve `e.into()` in the ack-wait loop; explicit `Box::new(e)` required (tungstenite::Error implements std::error::Error).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Added futures-util and native-tls as explicit Cargo.toml dependencies**
- **Found during:** Task 2 (build verification after writing src/main.rs)
- **Issue:** Plan noted futures-util and native-tls are "transitive dependencies" and "already available" without explicit declaration. In Rust 2021 edition, transitive dependencies cannot be imported directly in source code without being listed as direct dependencies in Cargo.toml. Build failed with E0432 (unresolved import `futures_util`) and E0433 (unresolved module `native_tls`).
- **Fix:** Added `futures-util = "0.3"` and `native-tls = "0.2"` to `[dependencies]` in Cargo.toml. Both versions match existing transitive pins (0.3.32, 0.2.18) so no new packages were resolved.
- **Files modified:** Cargo.toml
- **Verification:** cargo build exits 0 after fix
- **Committed in:** b24d5fc (Task 2 commit)

**2. [Rule 1 - Bug] Fixed type inference for tungstenite::Error in ack-wait loop**
- **Found during:** Task 2 (build verification)
- **Issue:** `return Err(e.into())` in the ack-wait loop produced E0282 (type annotations needed) — the compiler cannot infer the target type for `.into()` when the error type parameter is abstract.
- **Fix:** Replaced `e.into()` with `Box::new(e)` which is unambiguous since tungstenite::Error implements `std::error::Error + Send + Sync`.
- **Files modified:** src/ws.rs (2 occurrences)
- **Verification:** cargo build exits 0 after fix
- **Committed in:** b24d5fc (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (2 Rule 1 bugs)
**Impact on plan:** Both fixes necessary for compilation. The transitive-dep issue is a common Rust 2021 pitfall when the plan assumes re-export behavior from older editions. No scope creep.

## Issues Encountered
- Plan assumed futures-util and native-tls could be imported as transitive deps; Rust 2021 edition requires explicit direct declarations. Fixed via Rule 1.

## User Setup Required
Before running `cargo run`, ensure `.env` file at project root contains:

| Variable | Source |
|----------|--------|
| CSGOROLL_API_TOKEN | CSGORoll account settings / API section |
| CSGOROLL_SESSION | Browser DevTools after login: Application > Cookies > session value |
| CAPSOLVER_API_KEY | CapSolver Dashboard -> API Key (not used this phase but still validated at startup) |

Live connection test: `cargo run` should print `[ws] Connecting...`, `[ws] connection_ack received`, `[ws] Subscribe sent`, then `[trade]` lines as events arrive from the feed.

## Next Phase Readiness
- Trade struct is finalized with all 9 fields Phase 3 needs; connect_once signature is stable
- Phase 02-02 (reconnect loop) can wrap connect_once in a retry loop
- Phase 03 (trade filter) can take on_trade callback and apply whitelist/markup filter logic
- No blockers; cargo build is clean

---
*Phase: 02-websocket-connection*
*Completed: 2026-02-25*

## Self-Check: PASSED

- FOUND: src/ws.rs
- FOUND: src/main.rs
- FOUND: Cargo.toml
- FOUND: .planning/phases/02-websocket-connection/02-01-SUMMARY.md
- FOUND commit: 844d74a (feat(02-01): add serde deps and implement ws.rs)
- FOUND commit: b24d5fc (feat(02-01): wire connect_once into main.rs)
