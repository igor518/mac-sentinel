# Tasks: MCS-001

**Source**: canonical architecture, tests, and BDD state; use `tasks.trace.md` for canonical ownership and GREEN links.

## Scope Boundaries

- Rust owns collection, identity, alerts, storage, settings validation, and process actions.
- React owns presentation and user interaction through the injected bridge only.
- Tauri owns lifecycle, tray behavior, capabilities, and typed IPC.
- The first implemented milestone ends with read-only live monitoring and sustained CPU alerting.
- History, settings, memory-growth alerts, and termination follow in that order.

## Ownership Guardrails

| Critical Behavior | Owner | Overlap Response |
|-------------------|-------|------------------|
| Sampling and identity | `src-tauri/src/monitoring.rs` | Merge any competing sampler before feature work continues. |
| Alert state | `src-tauri/src/alerts.rs` | Rules may not be reimplemented in commands or React. |
| Persistence and retention | `src-tauri/src/storage.rs` | All SQLite access routes through the single storage owner. |
| Settings validation | `src-tauri/src/settings.rs` | React may display errors but cannot define validation rules. |
| Process actions | `src-tauri/src/termination.rs` | No shell command or PID-only signal path is permitted. |
| IPC | `src-tauri/src/commands.rs` and `src/bridge.ts` | No component imports Tauri APIs directly. |

## Constraint Coverage

| Constraint IDs | Tasks |
|----------------|-------|
| C1, C2 | Tasks 0, 1, 7, 8 |
| C3, C4 | Tasks 2, 3, 5 |
| C5, C8 | Tasks 4, 5 |
| C6 | Task 8 |
| C7 | Tasks 2, 7 |
| C9 | Tasks 1, 2 |
| C10 | Tasks 0, 8 |

## Milestones

| Milestone | Scenarios | Tasks | Checkpoint |
|-----------|-----------|-------|------------|
| M0: Tooling and walking skeleton | — | 0–1 | Runners work and the integrated shell builds. |
| M1: Live investigation | Dashboard, popover, process discovery, identity lifecycle | 2 | Live overview and process tests GREEN. |
| M2: CPU evidence alert | Sustained CPU alert | 3 | CPU alert journey GREEN. |
| M3: History and resilience | Dependency denial, sampling gap | 4 | Storage and resilience tests GREEN. |
| M4: User controls | Alert suppression, settings, deletion | 5 | Settings and history-control journey GREEN. |
| M5: Memory observation | Memory growth | 6 | Validated heuristic and alert suites GREEN. |
| M6: Safe action | Confirmed and prevented termination | 7 | Termination suites GREEN. |
| M7: Release evidence | — | 8 | Performance, privacy, license, and full regression gates pass. |

## Tasks

### Task 0: Bootstrap project, runners, configuration, and documentation

**Milestone**: M0 — Tooling and walking skeleton

**Structure**: `package.json`, `src-tauri/Cargo.toml`, and platform configuration

**Makes GREEN (Automated Tests)**:

- `TEST-security-boundary` → `src-tauri/tests/security.rs`

**Scope**: Establish reproducible Bun, Vite, Tauri, Cargo, Vitest, and WebdriverIO runners plus minimal entry stubs.
**Boundary**: No monitoring, alert, storage, settings, or termination behavior.

**Creates**:

- `index.html`
- `LICENSE`
- `README.md`
- `docs/metric-semantics.md`
- `docs/performance.md`
- `src-tauri/tauri.conf.json`
- `src-tauri/capabilities/default.json`
- `src-tauri/src/main.rs`
- `src-tauri/src/lib.rs`

**Modifies**:

- `package.json`
- `tsconfig.json`
- `vite.config.ts`
- `wdio.conf.ts`
- `src-tauri/Cargo.toml`
- `src-tauri/build.rs`
- `src/test/setup.ts`

**Must Not Touch**:

- Runtime monitoring or UI feature modules beyond empty entry stubs.

**Exclude**: No network capability, shell execution permission, privileged helper, or production WebDriver plugin.

**Anti-duplication**: Keep one manifest and one runner configuration per ecosystem.

**Duplication Guard**:

- Check existing manifests and generated Tauri files before creating replacements.
- Merge required settings into the canonical file instead of creating alternate configs.
- Verify no second application entry point is introduced.

**Verify**:

```bash
bun install
bunx vitest --version
bunx wdio --version
cargo test --manifest-path src-tauri/Cargo.toml --test security
```

**Done when**:

- [x] Dependency installation is reproducible from lockfiles.
- [x] Runner commands resolve without missing-module errors.
- [x] `TEST-security-boundary` is GREEN.
- [x] Bundle identifier and Apache-2.0 license are present.

### Task 1: Wire the Tauri and React walking skeleton

**Milestone**: M0 — Tooling and walking skeleton

**Structure**: Rust state and command boundary wired to an injected React bridge

**Enables (BDD)**:

- `dashboard_shows_current_evidence` (BR-1.1, BR-8.1, BR-8.2)
- `popover_summarizes_monitoring` (BR-1.2)

**Scope**: Create one compilable path through native entry, managed state, command boundary, bridge, hook, and dashboard shell.
**Boundary**: Return typed empty or unavailable state; no fake production metrics.

**Creates**:

- `src-tauri/src/model.rs`
- `src-tauri/src/commands.rs`
- `src-tauri/src/monitoring.rs`
- `src-tauri/src/platform/mod.rs`
- `src-tauri/src/platform/macos.rs`
- `src/main.tsx`
- `src/App.tsx`
- `src/types.ts`
- `src/bridge.ts`
- `src/useMonitoring.ts`
- `src/components/Dashboard.tsx`
- `src/styles.css`

**Modifies**:

- `src-tauri/src/main.rs`
- `src-tauri/src/lib.rs`

**Must Not Touch**:

- Alert, SQLite, settings, or termination implementation.

**Exclude**: No illustrative hard-coded process rows in production state.

**Anti-duplication**: Import all frontend Tauri access from `src/bridge.ts`; do not call Tauri APIs from components.

**Duplication Guard**:

- Confirm `commands.rs` is the only native IPC registration owner.
- Confirm one React root and one monitoring subscription exist.
- Stop and merge if a second snapshot type appears.

**Verify**:

```bash
cargo check --manifest-path src-tauri/Cargo.toml
bun run build
```

**Done when**:

- [x] Rust and TypeScript compile.
- [x] The window opens through the real Tauri entry point.
- [x] Empty and unavailable states render truthfully.
- [x] No duplicated bridge or application state exists.

### Task 2: Deliver live overview and process investigation

**Milestone**: M1 — Live investigation (BR-1, BR-2, BR-3, BR-8)

**Structure**: `src-tauri/src/monitoring.rs` through `src/components/ProcessExplorer.tsx`

**Makes GREEN (Automated Tests)**:

- `TEST-rust-monitoring` → `src-tauri/tests/monitoring.rs`
- `TEST-react-app` → `src/App.test.tsx`
- `TEST-e2e-overview` → `e2e/macsentinel.e2e.ts`
- `TEST-e2e-processes` → `e2e/macsentinel.e2e.ts`

**Makes GREEN (Behavior)**:

- `dashboard_shows_current_evidence`
- `popover_summarizes_monitoring`
- `processes_can_be_found_and_inspected`
- `process_identity_survives_lifecycle_changes`

**Scope**: Implement real `sysinfo` sampling, process identity, snapshot delivery, overview, search, sort, and process detail.
**Boundary**: Read-only monitoring only; no SQLite or process signal.

**Creates**:

- `src/components/ProcessExplorer.tsx`

**Modifies**:

- `src-tauri/src/model.rs`
- `src-tauri/src/monitoring.rs`
- `src-tauri/src/platform/mod.rs`
- `src-tauri/src/platform/macos.rs`
- `src-tauri/src/commands.rs`
- `src-tauri/tests/monitoring.rs`
- `src/types.ts`
- `src/bridge.ts`
- `src/useMonitoring.ts`
- `src/components/Dashboard.tsx`
- `src/App.tsx`
- `src/App.test.tsx`
- `e2e/macsentinel.e2e.ts`

**Must Not Touch**:

- Storage, settings, alert, or termination owners.

**Exclude**: No PID-only identity, fabricated zero, frontend polling loop, or CPU clamp at `100%` for processes.

**Anti-duplication**: Reuse the canonical `ProcessKey` and immutable snapshot across Rust, serialized types, and React.

**Duplication Guard**:

- Search for all sampling timers before adding the Tokio interval.
- Search for process identity structs before adding `ProcessKey`.
- Verify no component creates an independent snapshot cache.

**Verify**:

```bash
cargo test --manifest-path src-tauri/Cargo.toml --test monitoring
bun run test
bun run test:e2e -- --mochaOpts.grep="overview|process|popover"
```

**Done when**:

- [x] Monitoring tests are GREEN.
- [x] Four M1 BDD scenarios are GREEN.
- [x] Real execution shows current processes without administrator access.
- [x] Sampling gaps and unavailable fields remain explicit.
- [x] User-facing CPU percentages are rounded to the nearest whole percent without changing internal measurement precision.

### Task 3: Deliver sustained CPU alerting

**Milestone**: M2 — CPU evidence alert (BR-4.1)

**Structure**: `src-tauri/src/alerts.rs` through `src/components/AlertCenter.tsx`

**Makes GREEN (Behavior)**:

- `sustained_cpu_creates_evidence_alert`

**Enables (BDD)**:

- `alert_controls_prevent_noise` (BR-4.2)
- `memory_growth_reports_observation` (BR-4.3)

**Scope**: Add the CPU alert state machine, evidence model, current alert delivery, and alert presentation.
**Boundary**: CPU rule only; settings remain approved defaults.

**Creates**:

- `src-tauri/src/alerts.rs`
- `src/components/AlertCenter.tsx`

**Modifies**:

- `src-tauri/src/commands.rs`
- `src-tauri/tests/alerts.rs`
- `src/App.tsx`
- `e2e/macsentinel.e2e.ts`

**Must Not Touch**:

- SQLite schema, memory-growth heuristic, or termination.

**Exclude**: No root-cause claims and no continuity across a gap.

**Anti-duplication**: Rule state exists only in `alerts.rs`; React renders serialized evidence.

**Duplication Guard**:

- Check the monitoring owner for existing gap semantics.
- Import `ProcessKey`; do not define an alert-specific identity.
- Verify only one CPU-rule instance owns each process/rule pair.

**Verify**:

```bash
cargo test --manifest-path src-tauri/Cargo.toml --test alerts sustained_cpu
bun run test:e2e -- --mochaOpts.grep="sustained CPU"
```

**Done when**:

- [ ] The CPU scenario is GREEN.
- [ ] Threshold, duration, and observed value appear in evidence.
- [ ] A gap resets continuity.

### Task 4: Deliver history and resilient degraded monitoring

**Milestone**: M3 — History and resilience (BR-7.1, BR-7.3)

**Structure**: `src-tauri/src/storage.rs` and bounded persistence integration

**Makes GREEN (Automated Tests)**:

- `TEST-rust-storage` → `src-tauri/tests/storage.rs`
- `TEST-e2e-resilience` → `e2e/macsentinel.e2e.ts`

**Makes GREEN (Behavior)**:

- `dependency_denial_keeps_alerts_visible`
- `sampling_gap_resets_alert_continuity`

**Scope**: Add migrations, WAL storage, bounded writes, history queries, pruning, and degraded storage status.
**Boundary**: Storage cannot block or own live sampling.

**Creates**:

- `src-tauri/src/storage.rs`

**Modifies**:

- `src-tauri/src/monitoring.rs`
- `src-tauri/src/commands.rs`
- `src-tauri/tests/storage.rs`
- `src/useMonitoring.ts`
- `src/components/ProcessExplorer.tsx`
- `src/App.tsx`
- `e2e/macsentinel.e2e.ts`

**Must Not Touch**:

- Termination and memory-growth rule.

**Exclude**: No unbounded retry queue, silent database recreation, or deletion on corruption.

**Anti-duplication**: All SQLite operations route through the one storage owner.

**Duplication Guard**:

- Search for `rusqlite::Connection` outside `storage.rs` and remove duplicates.
- Confirm one migration version source exists.
- Confirm retention logic is not repeated in monitoring.

**Verify**:

```bash
cargo test --manifest-path src-tauri/Cargo.toml --test storage
bun run test:e2e -- --mochaOpts.grep="dependencies|sampling gap"
```

**Done when**:

- [ ] Storage and resilience tests are GREEN.
- [ ] History separates reused PIDs.
- [ ] Live monitoring continues during storage failure.

### Task 5: Deliver settings, alert noise controls, and history deletion

**Milestone**: M4 — User controls (BR-4.2, BR-5)

**Structure**: `src-tauri/src/settings.rs` through `src/components/SettingsView.tsx`

**Makes GREEN (Automated Tests)**:

- `TEST-e2e-settings` → `e2e/macsentinel.e2e.ts`

**Makes GREEN (Behavior)**:

- `alert_controls_prevent_noise`
- `settings_and_deletion_change_future_behavior`

**Scope**: Validate and persist settings, apply cooldowns and allowlists, reflect notifications, and delete retained history after confirmation.
**Boundary**: No termination or experimental memory rule.

**Creates**:

- `src-tauri/src/settings.rs`
- `src/components/SettingsView.tsx`

**Modifies**:

- `src-tauri/src/storage.rs`
- `src-tauri/src/alerts.rs`
- `src-tauri/src/commands.rs`
- `src/components/AlertCenter.tsx`
- `src/App.tsx`
- `e2e/macsentinel.e2e.ts`

**Must Not Touch**:

- Process signal paths.

**Exclude**: No frontend-only validation or silent fallback for malformed saved settings.

**Anti-duplication**: Rust settings are canonical; TypeScript mirrors values for display only.

**Duplication Guard**:

- Search for default constants before defining settings defaults.
- Confirm alert cooldown and allowlist checks remain in `alerts.rs`.
- Confirm history deletion remains a storage command.

**Verify**:

```bash
cargo test --manifest-path src-tauri/Cargo.toml --test settings_actions default_settings aggregate_retention
bun run test:e2e -- --mochaOpts.grep="settings|deletes retained history|suppresses"
```

**Done when**:

- [ ] Settings and deletion scenario is GREEN.
- [ ] Alert suppression scenario is GREEN.
- [ ] Saved values affect subsequent samples and rules.

### Task 6: Validate and deliver memory-growth alerting

**Milestone**: M5 — Memory observation (BR-4.3)

**Structure**: Experimental evidence in `docs/performance.md` implemented by `src-tauri/src/alerts.rs`

**Makes GREEN (Automated Tests)**:

- `TEST-rust-alerts` → `src-tauri/tests/alerts.rs`
- `TEST-e2e-alerts` → `e2e/macsentinel.e2e.ts`

**Makes GREEN (Behavior)**:

- `memory_growth_reports_observation`

**Scope**: Record representative workloads, choose a bounded memory-growth rule, document tradeoffs, then implement it.
**Boundary**: The rule remains disabled until evidence and test cases exist.

**Creates**:

- None.

**Modifies**:

- `docs/performance.md`
- `src-tauri/src/alerts.rs`
- `src-tauri/tests/alerts.rs`
- `src/components/AlertCenter.tsx`
- `e2e/macsentinel.e2e.ts`

**Must Not Touch**:

- Collector normalization, storage schema, or termination.

**Exclude**: No linear-growth assumption without a defined window and minimum delta.

**Anti-duplication**: Extend the canonical alert engine; do not add a second scheduler.

**Duplication Guard**:

- Reuse alert evidence and cooldown types.
- Confirm memory state is keyed by `ProcessKey`.
- Verify rule configuration is not duplicated in React.

**Verify**:

```bash
cargo test --manifest-path src-tauri/Cargo.toml --test alerts
bun run test:e2e -- --mochaOpts.grep="alert|memory"
```

**Done when**:

- [ ] The experiment and selected rule are documented.
- [ ] Alert suites are GREEN.
- [ ] Memory alerts state observations without causal claims.

### Task 7: Deliver confirmation-gated safe termination

**Milestone**: M6 — Safe action (BR-6)

**Structure**: `src-tauri/src/termination.rs` through the process detail confirmation UI

**Makes GREEN (Automated Tests)**:

- `TEST-rust-settings-actions` → `src-tauri/tests/settings_actions.rs`
- `TEST-e2e-termination` → `e2e/macsentinel.e2e.ts`

**Makes GREEN (Behavior)**:

- `confirmed_termination_reports_outcome`
- `unsafe_termination_is_prevented`

**Scope**: Revalidate identity and ownership after confirmation, send `SIGTERM`, and return typed outcomes.
**Boundary**: Eligible user-owned processes only; no escalation or automation.

**Creates**:

- `src-tauri/src/termination.rs`

**Modifies**:

- `src-tauri/src/platform/macos.rs`
- `src-tauri/src/commands.rs`
- `src-tauri/tests/settings_actions.rs`
- `src/components/ProcessExplorer.tsx`
- `src/App.tsx`
- `e2e/macsentinel.e2e.ts`

**Must Not Touch**:

- Shell execution permissions or privileged helpers.

**Exclude**: No PID-only signal, `SIGKILL` escalation, or automatic termination.

**Anti-duplication**: All process signals route through `TerminationService` and the platform adapter.

**Duplication Guard**:

- Search for every `kill` or signal call before implementation.
- Confirm post-confirmation identity validation is shared by every outcome path.
- Verify React never invokes a shell command.

**Verify**:

```bash
cargo test --manifest-path src-tauri/Cargo.toml --test settings_actions
bun run test:e2e -- --mochaOpts.grep="termination"
```

**Done when**:

- [ ] Termination tests and both BDD scenarios are GREEN.
- [ ] Identity mismatch sends no signal.
- [ ] Every failure has a distinct user-visible result.

### Task 8: Prove performance, privacy, licensing, and release documentation

**Milestone**: M7 — Release evidence

**Structure**: Verification and documentation artifacts

**Makes GREEN (Automated Tests)**:

- `TEST-performance-budget` → `src-tauri/tests/performance.rs`
- `TEST-security-boundary` → `src-tauri/tests/security.rs`

**Scope**: Run the reference-Mac benchmark, observe network behavior, complete documentation, and execute all regression gates.
**Boundary**: Evidence and corrective fixes only; no new capability.

**Creates**:

- None.

**Modifies**:

- `src-tauri/tests/performance.rs`
- `src-tauri/tests/security.rs`
- `docs/performance.md`
- `docs/metric-semantics.md`
- `README.md`
- `LICENSE`
- `src-tauri/tauri.conf.json`
- `src-tauri/capabilities/default.json`

**Must Not Touch**:

- Product scope or canonical measurement semantics without a new approved requirement.

**Exclude**: No signing, notarization, port inspector, project profiles, or AI diagnostics.

**Anti-duplication**: Update the canonical docs and tests; do not create parallel release notes for the same evidence.

**Duplication Guard**:

- Search existing metric and performance documentation before adding evidence.
- Confirm each command appears once in README guidance.
- Verify no secondary capabilities file broadens permissions.

**Verify**:

```bash
cargo test --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml --test performance -- --ignored
bun run test
bun run build
bun run test:e2e
```

**Done when**:

- [ ] CPU and memory budgets pass on the reference Mac.
- [ ] Normal operation produces no monitoring-data network request.
- [ ] All Rust, React, and integrated E2E suites are GREEN.
- [ ] Metric, privacy, permission, retention, and termination limitations are documented.

## Architecture Coverage

| Layer | Arch Files | In Tasks | Gap | Status |
|-------|-----------:|---------:|----:|--------|
| Root configuration and docs | 9 | 9 | 0 | ✅ |
| Tauri configuration | 4 | 4 | 0 | ✅ |
| Rust runtime | 11 | 11 | 0 | ✅ |
| Rust tests | 6 | 6 | 0 | ✅ |
| Frontend runtime | 10 | 10 | 0 | ✅ |
| Frontend and E2E tests | 3 | 3 | 0 | ✅ |

### Orphaned Files

- None.

## Post-Implementation

- [ ] No duplicated runtime owner exists.
- [ ] All Rust tests are GREEN.
- [ ] All React tests are GREEN.
- [ ] All 12 BDD scenarios are GREEN through integrated E2E coverage.
- [ ] Reference-Mac performance and privacy evidence is recorded.
- [ ] Degraded and fallback paths match requirements.

## Post-Verify Fixes

- Add only if completion review finds critical or high-severity issues.
