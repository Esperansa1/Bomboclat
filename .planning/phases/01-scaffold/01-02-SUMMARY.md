---
phase: 01-scaffold
plan: 02
subsystem: infra
tags: [rust, config, credentials, dotenvy, env-validation]

# Dependency graph
requires:
  - "01-01: Compilable Cargo.toml with dotenvy dependency locked"
provides:
  - "src/config.rs: Config struct with load() function for startup credential validation"
  - ".env.example: Template documenting all three required environment variable names"
  - "Startup fails fast with named error on any missing credential"
affects: [03-websocket, 04-http-buy, 05-snipe-engine]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "dotenvy::dotenv().ok() called before std::env::var - loads .env silently, does not fail if missing"
    - "process::exit(1) with eprintln! to stderr for fast-fail credential validation"
    - "_config prefix retains Config value without unused-variable warning; not dropped immediately"

key-files:
  created:
    - "src/config.rs - Config struct with three String credential fields and load() + require_var() impl"
    - ".env.example - Template with CSGOROLL_API_TOKEN, CSGOROLL_SESSION, CAPSOLVER_API_KEY documented"
  modified:
    - "src/main.rs - Added mod config; let _config = config::Config::load(); replaces TODO placeholder"

key-decisions:
  - "Used _config (not let _ = ...) to retain Config value in scope for Phase 2 WebSocket use"
  - "dead_code warning on Config fields is expected - fields are pub and will be consumed in Phase 2; not suppressed"

requirements-completed: [CONN-02, PROT-03]

# Metrics
duration: 5min
completed: 2026-02-24
---

# Phase 1 Plan 02: Credential loading with startup validation - Config struct reads CSGOROLL_API_TOKEN, CSGOROLL_SESSION, CAPSOLVER_API_KEY from env/.env and exits 1 with named error on any missing variable

**src/config.rs module with Config struct and load() function validates all three credentials at startup using dotenvy + std::env::var; binary exits 0 with masked confirmation or exits 1 naming the specific missing variable**

## Performance

- **Duration:** 5 min
- **Started:** 2026-02-24T20:28:46Z
- **Completed:** 2026-02-24T20:33:00Z
- **Tasks:** 2
- **Files modified:** 3 (created: src/config.rs, .env.example; modified: src/main.rs)

## Accomplishments
- src/config.rs created with `pub struct Config` holding three String credential fields
- Config::load() calls dotenvy::dotenv().ok() first, then reads each var via std::env::var
- require_var() exits process with code 1 and names the specific missing/empty variable on any error
- No credential values logged in plaintext - only `***` masked confirmation printed to stdout
- src/main.rs updated: `mod config;` declared, `let _config = config::Config::load()` called at top of main before any other work
- .env.example created at project root documenting all three required variable names with source comments
- All three acceptance scenarios verified: all-present exits 0, missing CSGOROLL_SESSION exits 1, missing CAPSOLVER_API_KEY exits 1

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement config module with credential loading and startup validation** - `d3c9a69` (feat)
2. **Task 2: Wire config into main.rs and create .env.example** - `efe8b7f` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified

- `src/config.rs` - Config struct with `pub csgoroll_api_token`, `pub csgoroll_session`, `pub capsolver_api_key` String fields; `pub fn load() -> Self`; private `fn require_var(name: &str) -> String`
- `src/main.rs` - `mod config;` at top; `let _config = config::Config::load();` as first statement in main(); old TODO placeholder removed
- `.env.example` - Template with all three variable names and source documentation comments

## Config Module Public API

```rust
pub struct Config {
    pub csgoroll_api_token: String,
    pub csgoroll_session: String,
    pub capsolver_api_key: String,
}

impl Config {
    pub fn load() -> Self { ... }
}
```

## Startup Behavior Verification

| Scenario | Input | Output | Exit Code |
|----------|-------|--------|-----------|
| All credentials set | CSGOROLL_API_TOKEN=tok CSGOROLL_SESSION=sess CAPSOLVER_API_KEY=cap | `[config] All credentials loaded: CSGOROLL_API_TOKEN=***, CSGOROLL_SESSION=***, CAPSOLVER_API_KEY=***` + `CSGORoll Sniper ready.` | 0 |
| Missing CSGOROLL_SESSION | CSGOROLL_API_TOKEN=tok CAPSOLVER_API_KEY=cap | `[config] ERROR: Required environment variable 'CSGOROLL_SESSION' is not set...` | 1 |
| Missing CAPSOLVER_API_KEY | CSGOROLL_API_TOKEN=tok CSGOROLL_SESSION=sess | `[config] ERROR: Required environment variable 'CAPSOLVER_API_KEY' is not set...` | 1 |

## Compiler Warnings

One warning was present throughout: `fields csgoroll_api_token, csgoroll_session, and capsolver_api_key are never read`. This is expected - the fields are declared `pub` and will be consumed in Phase 2 (WebSocket authentication). The warning was not suppressed via `#[allow(dead_code)]` because it is a valid reminder that will resolve naturally in Phase 3 plan 01 when the Config struct is first used.

## Decisions Made

- **_config binding retained:** `let _config = config::Config::load()` keeps the Config value alive for the scope of main(). `let _ = ...` would drop it immediately. Phase 2 will rename this to `config` when the fields are first accessed.
- **dead_code warning not suppressed:** The warning is benign and self-documenting. Adding `#[allow(dead_code)]` would hide a legitimate reminder.

## Deviations from Plan

None - plan executed exactly as written.

## User Setup Required

Before running the binary in production, copy `.env.example` to `.env` and populate:

| Variable | Source |
|----------|--------|
| CSGOROLL_API_TOKEN | CSGORoll account settings or developer/API section |
| CSGOROLL_SESSION | Browser DevTools after login: Application > Cookies |
| CAPSOLVER_API_KEY | CapSolver Dashboard (https://dashboard.capsolver.com) -> API Key |

## Next Phase Readiness

- Config struct is available; all three credentials loaded and validated at startup
- Phase 2 (WebSocket connection) can access credentials via `config.csgoroll_api_token` and `config.csgoroll_session`
- Phase 4 (HTTP buy) can access `config.capsolver_api_key` for CapSolver challenge resolution

---
*Phase: 01-scaffold*
*Completed: 2026-02-24*

## Self-Check: PASSED

- FOUND: src/config.rs
- FOUND: src/main.rs
- FOUND: .env.example
- FOUND: .planning/phases/01-scaffold/01-02-SUMMARY.md
- FOUND commit: d3c9a69 (feat(01-02): config module)
- FOUND commit: efe8b7f (feat(01-02): main.rs + .env.example)
