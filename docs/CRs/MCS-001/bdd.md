# BDD: MCS-001

**Source**: [MCS-001](../MCS-001-process-monitor.md)
**Generated**: 2026-11-07
**Mode**: Specification only; no executable E2E framework exists yet.

## Journey Groups

| Journey | Scenario Focus | Delivery Slice |
|---------|----------------|----------------|
| Live awareness | Dashboard, menu-bar popover, metric semantics, accessible status | 1 |
| Process investigation | Search, sorting, details, lifecycle identity | 1 and 2 |
| Resource alerts | Sustained CPU, suppression, memory-growth observation | 1, 3, and 4 |
| User control | Settings, retention deletion, notification fallback | 3 |
| Safe action | Confirmed termination and cancellation or identity-race prevention | 5 |
| Resilience | Storage degradation and sampling gaps | 2 and 3 |

## Acceptance Notes

- Every scenario is expressed through user-visible behavior; internal module names and storage mechanics are excluded.
- Alert scenarios describe observed evidence and never claim an unverified cause.
- Process termination is deliberately separated into successful confirmation and prevention paths.
- Dependency-denial behavior keeps the live-monitoring promise visible rather than silently degrading.
- Sampling gaps break sustained-alert continuity to prevent false positives after sleep or delay.

## Executable E2E Decision

- Architecture must select a macOS-capable Tauri E2E strategy before `/mdt:tests` completes.
- The selected runner must exercise the integrated application rather than isolated React components alone.
- Until the runner exists, `bdd.trace.md` is the acceptance specification and no executable-coverage claim is made.

## Review Gate

- Scenario count: 12 of 12 permitted.
- Each BDD-routed behavior is covered by at least one canonical scenario.
- Constraints and edge-case IDs remain routed to architecture and tests rather than BDD.
- No new behavior was introduced downstream of requirements.

---

Use `bdd.trace.md` for canonical scenario text and requirement coverage.
