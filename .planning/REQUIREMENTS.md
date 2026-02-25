# Requirements: CSGORoll Skin Sniper

**Defined:** 2026-02-24
**Core Value:** React to a new whitelisted listing before any other buyer does — the engine wins when it snipes items that would otherwise be taken by competitors within milliseconds.

## v1 Requirements

### Connection (CONN)

- [x] **CONN-01**: Bot connects to CSGORoll's WebSocket feed and maintains a persistent, auto-reconnecting connection with exponential backoff
- [x] **CONN-02**: Bot loads auth credentials (API token / session cookie) from environment variables or .env file at startup
- [ ] **CONN-03**: Bot uses a single pre-authenticated keep-alive HTTP/1.1 connection for buy requests (no TLS handshake per order)

### Protection (PROT)

- [ ] **PROT-01**: Bot uses CapSolver to resolve Cloudflare challenges on the HTTP buy endpoint
- [ ] **PROT-02**: Bot uses CapSolver to resolve any captcha challenges during buy request flow
- [x] **PROT-03**: CapSolver API key is loaded from environment variables or .env file at startup

### Matching (MATCH)

- [ ] **MATCH-01**: Bot pre-loads item whitelist from JSON config at startup into an in-memory O(1) lookup (AHashMap)
- [ ] **MATCH-02**: Whitelist entries match on exact combination of skin name × wear tier × StatTrak flag
- [ ] **MATCH-03**: Bot supports all five wear tiers: Battle-Scarred, Well-Worn, Field-Tested, Minimal Wear, Factory New
- [ ] **MATCH-04**: Bot evaluates each incoming listing against whitelist and per-item markup threshold with zero disk I/O
- [ ] **MATCH-05**: Each whitelist entry supports configurable min/max price bounds alongside markup threshold

### Trading (TRADE)

- [ ] **TRADE-01**: Bot fires buy API request immediately on a whitelist match
- [ ] **TRADE-02**: Bot tracks current coin balance and session inventory (list of items bought) in memory
- [ ] **TRADE-03**: Bot skips a buy attempt silently if balance is insufficient for that specific item's price
- [ ] **TRADE-04**: Bot exits the process cleanly when balance drops below the configured global minimum purchase price

### Observability (OBS)

- [ ] **OBS-01**: Bot logs buy wins, misses, and errors to a persistent store asynchronously outside the hot path
- [ ] **OBS-02**: Bot measures and records end-to-end latency (WS event received → buy request sent) for each match event

## v2 Requirements

### Notifications

- **NOTIF-01**: Bot sends Telegram or Discord alert on successful buy
- **NOTIF-02**: Bot sends alert when balance drops below warning threshold (before process exit)

### Multi-platform

- **PLAT-01**: Support additional skin trading platforms (DMarket, Skinport)

### Portfolio

- **PORT-01**: Track purchase price and timestamp per item (cost basis)
- **PORT-02**: Export session P&L report on exit

## Out of Scope

| Feature | Reason |
|---------|--------|
| Multi-account operation | Single account only; multi-account adds complexity with no v1 benefit |
| External price validation (Steam median, CSGOFloat) | Eliminates external API latency from hot path; markup from CSGORoll feed is the sole signal |
| Web UI or dashboard | Config via files, monitoring via logs |
| Mobile alerts (Telegram/Discord) | v2 if needed |
| Portfolio management / P&L tracking | Not core to trading engine; v2 if needed |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CONN-01 | Phase 2 | Complete |
| CONN-02 | Phase 1 | Complete |
| CONN-03 | Phase 4 | Pending |
| PROT-01 | Phase 4 | Pending |
| PROT-02 | Phase 4 | Pending |
| PROT-03 | Phase 1 | Complete |
| MATCH-01 | Phase 3 | Pending |
| MATCH-02 | Phase 3 | Pending |
| MATCH-03 | Phase 3 | Pending |
| MATCH-04 | Phase 3 | Pending |
| MATCH-05 | Phase 3 | Pending |
| TRADE-01 | Phase 4 | Pending |
| TRADE-02 | Phase 5 | Pending |
| TRADE-03 | Phase 5 | Pending |
| TRADE-04 | Phase 5 | Pending |
| OBS-01 | Phase 6 | Pending |
| OBS-02 | Phase 6 | Pending |

**Coverage:**
- v1 requirements: 17 total
- Mapped to phases: 17
- Unmapped: 0

---
*Requirements defined: 2026-02-24*
*Last updated: 2026-02-24 — CONN-02, PROT-03 marked complete after plan 01-01*
