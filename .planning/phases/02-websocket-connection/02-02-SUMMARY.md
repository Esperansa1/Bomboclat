---
phase: 02-websocket-connection
plan: 02
subsystem: infra
tags: [rust, tokio, websocket, exponential-backoff, reconnect]

# Dependency graph
requires:
  - phase: 02-websocket-connection
    plan: 01
    provides: "src/ws.rs: connect_once(config, on_trade) returning Result; Trade struct with 9 fields"
provides:
  - "src/ws.rs: pub async fn run_with_reconnect(config, on_trade) — persistent reconnect loop with exponential backoff"
  - "src/main.rs: persistent WebSocket driver calling run_with_reconnect (Phase 3 replaces closure)"
affects: [03-trade-filter, 04-http-buy, 05-snipe-engine]

# Tech tracking
tech-stack:
  added:
    - "tokio::time::sleep — async sleep for backoff delays (already in tokio::full feature)"
    - "std::time::Duration — Duration::from_secs for sleep argument (stdlib, no new dep)"
  patterns:
    - "Reconnect wrapper pattern: run_with_reconnect calls connect_once in a loop, owns backoff state"
    - "Productive-session reset: backoff resets to 1s only when received_count > 0 — failed fast connections double delay"
    - "FnMut callback threading: run_with_reconnect takes mut on_trade and re-passes to connect_once inner closure by reference"

key-files:
  created: []
  modified:
    - "src/ws.rs - Added pub async fn run_with_reconnect with exponential backoff loop; added use std::time::Duration"
    - "src/main.rs - Replaced connect_once one-shot call with run_with_reconnect persistent loop"

key-decisions:
  - "run_with_reconnect wraps connect_once rather than inlining connection logic: preserves single-responsibility; connect_once remains testable in isolation"
  - "Backoff advances AFTER sleep, not before: the current delay_secs is used for this sleep, then doubled for the next failure"
  - "received_count tracked inside loop per-attempt via closure capture: clean reset semantics without external state"

patterns-established:
  - "Reconnect loop pattern: loop { result = connect_once().await; backoff-sleep; advance-delay-if-no-progress }"
  - "Phase 3 integration point: replace println! closure in main.rs with whitelist-filter closure — no ws.rs changes needed"

requirements-completed: [CONN-01]

# Metrics
duration: 3min
completed: 2026-02-25
---

# Phase 2 Plan 02: Exponential Backoff Reconnect Wrapper — run_with_reconnect making WebSocket connection self-healing

**Persistent WebSocket reconnect loop with 1s->2s->4s->8s->16s->32s->60s exponential backoff, automatic reset after productive sessions, wired as the production entry point in main.rs**

## Performance

- **Duration:** 3 min
- **Started:** 2026-02-25T05:45:37Z
- **Completed:** 2026-02-25T05:48:38Z
- **Tasks:** 2
- **Files modified:** 2 (modified: src/ws.rs, src/main.rs)

## Accomplishments
- `pub async fn run_with_reconnect` added to src/ws.rs: wraps connect_once in an infinite loop with exponential backoff (1s cap 60s)
- Backoff resets to 1s after any connection that delivered at least one trade (received_count > 0 guard)
- Both clean close (Ok) and errors (Err) are handled: logs reconnect delay to stdout/stderr respectively
- src/main.rs updated from one-shot connect_once to persistent run_with_reconnect — bot now survives any disconnect
- cargo build exits 0 with zero errors; 2 pre-existing dead_code warnings for future-phase fields unchanged

## Task Commits

Each task was committed atomically:

1. **Task 1: Add run_with_reconnect with exponential backoff to ws.rs** - `49d0bfa` (feat)
2. **Task 2: Update main.rs to use run_with_reconnect as persistent driver** - `6edc37d` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified
- `src/ws.rs` - Added `use std::time::Duration`; appended `pub async fn run_with_reconnect` with backoff constants, received_count tracking, sleep, delay doubling
- `src/main.rs` - Replaced `match ws::connect_once(...)` block with `ws::run_with_reconnect(...).await`; updated startup log message

## Decisions Made
- **run_with_reconnect wraps connect_once rather than inlining:** Single-responsibility principle preserved; connect_once remains independently usable and testable. Phase 3 does not need to know about reconnect logic.
- **Backoff delay is applied to current failure, then advanced:** `sleep(delay_secs)` happens before `delay_secs *= 2`. This means the first failure waits 1s, next waits 2s, etc. — matches the documented "1s, 2s, 4s" schedule in the plan.
- **received_count reset semantics:** Only resets `delay_secs` to 1 if the connection that just closed was productive. A connection that connected but delivered 0 trades (e.g. auth issue returning close immediately) still advances the backoff.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Added `use std::time::Duration` import**
- **Found during:** Task 1 (writing run_with_reconnect)
- **Issue:** Plan noted "verify `use std::time::Duration` is already there from Task 1 of plan 02-01" — it was not present in ws.rs. `tokio::time::sleep` requires a `Duration` argument; without the import the build would fail.
- **Fix:** Added `use std::time::Duration;` to the imports block at the top of src/ws.rs
- **Files modified:** src/ws.rs
- **Verification:** cargo build exits 0 after addition
- **Committed in:** 49d0bfa (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 Rule 2 — missing import for correctness)
**Impact on plan:** Trivial one-line import addition necessary for compilation. No scope creep.

## Issues Encountered
- Plan assumed `use std::time::Duration` was already present from 02-01; it was not. Added as Rule 2 deviation during Task 1.

## User Setup Required
None - no external service configuration required for this plan. Existing `.env` from 02-01 setup still applies.

## Next Phase Readiness
- run_with_reconnect signature is stable: `pub async fn run_with_reconnect(config: &Config, on_trade: impl FnMut(Trade))`
- Phase 3 (trade filter) replaces the println! closure in main.rs with whitelist-evaluating logic — no changes to ws.rs needed
- Backoff behavior is observable in stdout/stderr on reconnect events
- No blockers; cargo build is clean

---
*Phase: 02-websocket-connection*
*Completed: 2026-02-25*

## Self-Check: PASSED

- FOUND: src/ws.rs
- FOUND: src/main.rs
- FOUND: .planning/phases/02-websocket-connection/02-02-SUMMARY.md
- FOUND commit: 49d0bfa (feat(02-02): add run_with_reconnect with exponential backoff to ws.rs)
- FOUND commit: 6edc37d (feat(02-02): update main.rs to use run_with_reconnect as persistent driver)
