# MacSentinel MVP Concept

## Product Promise

- Help developers understand what their Mac is doing, why it is slowing down, and which processes remain active.
- Combine live process metrics with short-term history and evidence-based anomaly alerts.
- Keep process data local, require no account, and send no telemetry during normal operation.

## Primary Users

- Developers running editors, build tools, containers, databases, and local servers.
- MacBook users investigating sustained CPU or memory use.
- Technical users who prefer transparent, local-first diagnostics.

## MVP Journeys

1. Open the menu-bar popover and see current CPU, memory, process count, top processes, and active alerts.
2. Open the dashboard and search or sort the process list.
3. Select a process and inspect recent CPU and resident-memory history.
4. Receive a sustained-resource alert that states the observed metric, threshold, and duration.
5. Adjust sampling, retention, thresholds, notifications, startup behavior, cooldowns, and allowlists.
6. Confirm termination of an eligible user-owned process and receive a clear result.
7. Delete locally retained monitoring history.

## MVP Boundary

### Included

- macOS menu-bar app and main dashboard.
- Live system and process CPU and memory observations.
- Stable process identity using PID plus process start time.
- Short-term local history and bounded SQLite retention.
- Sustained CPU and memory-growth alerts.
- Confirmation-gated termination of eligible processes.
- Local settings and privacy controls.

### Deferred

- Port inspector and project profiles.
- Process relationship visualization.
- Docker integration.
- Learned baselines and AI-generated summaries.
- Public signing, notarization, and distribution packaging.
- Intel Mac support beyond best-effort compatibility.

## Delivery Order

1. Rust collector, stable process identity, dashboard, ranked processes, and sustained CPU alert.
2. Short-term history and bounded SQLite persistence.
3. Settings, retention controls, notification controls, cooldowns, and allowlists.
4. Experimentally validated memory-growth alert.
5. Confirmation-gated process termination after read-only monitoring is stable.

## Measurement Semantics

- System CPU: `100%` represents all logical cores fully utilized.
- Process CPU: `100%` represents one logical core; multi-threaded processes may exceed `100%`.
- Default sampling interval: 2 seconds.
- Default sustained CPU alert: 80% of one logical core for 10 minutes.
- Default raw-sample retention: 24 hours.
- Default aggregate retention: 7 days.
- Idle target: less than 1% CPU and 100 MB resident memory on the supported reference Mac.
- Reference Mac: MacBook Pro `Mac16,7`, Apple M4 Pro, 14 CPU cores, and 48 GB RAM.

## Safety and Privacy

- Core monitoring works without administrator privileges.
- Inaccessible process fields are presented as unavailable, not fabricated as zero.
- Alerts report observations and avoid claiming an unproven root cause.
- No process is terminated automatically.
- Termination requires identity revalidation and explicit confirmation.
- Monitoring data remains on the device unless a later, separate feature adds explicit export or integration.

## Visual Contract

- Main dashboard follows the supplied 2026-11-06 visual reference.
- Surfaces are near-black with restrained borders and generous spacing.
- System-level metrics use green accents.
- Per-process bars use blue accents.
- Warnings use orange accents and pair color with text or iconography.
- Typography favors high-contrast numeric readings and compact supporting labels.
- The menu-bar popover is a reduced derivative of the dashboard, not a separate visual system.
- Accessibility includes keyboard operation, VoiceOver labels, visible focus, and non-color-only status cues.

## Technical Direction

- Desktop shell: Tauri 2.
- Application core: Rust.
- UI: React and TypeScript.
- Monitoring: `sysinfo` plus targeted macOS APIs where validated.
- Background scheduling: Tokio.
- Persistence: SQLite through an architecture-selected Rust client.
- Notifications and autostart: supported Tauri plugins after capability review.
- Sampling stays in Rust; the frontend consumes typed snapshots and commands.
- Bundle identifier: `com.macsentinel.app`.
- License: Apache License 2.0.

## Source Material

- Original concept: `/Users/ihorchernin/Downloads/MacSentinel_Project_Idea.md`.
- Original visual: MacSentinel dashboard screenshot supplied on 2026-11-06.

