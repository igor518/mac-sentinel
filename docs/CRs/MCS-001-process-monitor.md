---
code: MCS-001
status: In Progress
dateCreated: 2026-11-06T22:58:39.585Z
type: Feature Enhancement
priority: High
---

# Build privacy-first macOS process monitor

## 1. Description

### Requirements Scope

`full`

### Problem

- Developers cannot easily reconstruct transient CPU and memory problems after a spike ends.
- Forgotten local services, containers, and build processes can consume resources without clear historical context.
- Existing macOS tools require switching between snapshots and terminal commands to understand sustained process behavior.

### Affected Areas

- Desktop experience: menu-bar popover, dashboard, process explorer, history, alerts, and settings.
- Monitoring domain: system and per-process CPU, memory, lifecycle, and identity observations.
- Local data: bounded short-term samples, aggregated history, alert events, retention, and deletion.
- macOS integration: process metadata, notifications, startup behavior, and confirmation-gated termination.
- Quality: privacy, least privilege, energy use, measurement semantics, and failure transparency.

### Scope

- In scope:
	- macOS 14+ on Apple Silicon for the initial supported target.
	- Live system CPU, memory, active-process count, and top-process views.
	- Searchable and sortable process exploration with recent CPU and memory history.
	- Stable process identity based on PID plus process start time.
	- Local SQLite history with configurable retention.
	- Sustained CPU and memory-growth alerts with cooldowns and allowlists.
	- Explicitly confirmed termination for eligible user-owned processes.
	- Menu-bar popover plus the dark dashboard visual direction supplied with the concept.
- Out of scope:
	- Antivirus, EDR, firewall, or automated threat claims.
	- Automatic process termination.
	- Cloud accounts, telemetry, or silent data upload.
	- Exact battery-impact or network-bandwidth claims unsupported by macOS data.
	- Port inspection, project profiles, process trees, Docker integration, baselines, AI summaries, signing, and notarized distribution.

## 2. Desired Outcome

### Success Conditions

- Users can identify which processes currently consume the most CPU and resident memory.
- Users can inspect recent process trends after a transient resource spike.
- Users receive an evidence-based alert after a process sustains configured resource use.
- Users can distinguish observed metrics from diagnostic inferences.
- Users can manage sampling, retention, notifications, startup, allowlists, and thresholds locally.
- Users can terminate an eligible process only after explicit confirmation and receive a clear outcome.

### Constraints

- Core monitoring must work without administrator privileges.
- Process names, paths, samples, settings, and alerts must remain on the Mac by default.
- Sampling defaults to 2 seconds and must remain user-adjustable.
- A default CPU alert represents 80% of one logical core sustained for 10 minutes.
- System CPU uses total-machine normalization; process CPU uses one-logical-core normalization and may exceed 100% for multi-threaded work.
- Raw samples retain for 24 hours and aggregates retain for 7 days by default.
- Idle overhead target is less than 1% CPU and 100 MB resident memory on the supported reference Mac.
- The app must tolerate inaccessible and short-lived processes without crashing or presenting fabricated zero values.
- The visual system uses a minimalist black surface, green system metrics, blue process bars, and orange warnings.

### Non-Goals

- Replacing all Activity Monitor capabilities.
- Diagnosing an unobserved root cause as fact.
- Requiring kernel extensions, reduced macOS protections, or elevated privileges for core monitoring.
- Supporting Windows or Linux in the first release.

## 3. Open Questions

| Area | Question | Constraints |
|------|----------|-------------|
| Collection | Which Rust library and targeted macOS APIs provide accurate metrics with acceptable overhead? | Must preserve least privilege and documented normalization. |
| Storage | Which SQLite client and aggregation schedule best satisfy bounded retention? | Must remain fully local and support deletion. |
| UI bridge | Which typed event and command boundary keeps sampling outside React? | Menu-bar rendering must remain lightweight. |
| Verification | Which repeatable workloads establish CPU accuracy and idle-overhead evidence? | Results must be recorded on MacBook Pro `Mac16,7`, Apple M4 Pro, 14 CPU cores, and 48 GB RAM. |
| Distribution | Which signing and notarization work belongs after the local MVP? | Distribution hardening is not part of this ticket. |

### Known Constraints

- The initial stack is Rust, Tauri 2, React, and TypeScript.
- The initial target is macOS 14+ on Apple Silicon; Intel support is best-effort.
- The initial release is local-first and account-free.
- The supplied visual is the design baseline for the main dashboard; the popover is a compact derivative.

### Approved Product Decisions

- Delivery proceeds through a narrow first milestone: collector, stable process identity, dashboard, ranked processes, and sustained CPU alert.
- History, settings, memory-growth alerts, and termination follow as incremental vertical slices.
- Confirmation-gated termination remains in the MVP but is implemented only after read-only monitoring is stable.
- Architecture may define and validate the memory-growth heuristic experimentally before locking its default parameters.
- Performance is benchmarked on the current reference Mac: MacBook Pro `Mac16,7`, Apple M4 Pro, 14 CPU cores, and 48 GB RAM.
- The application bundle identifier is `com.macsentinel.app`.
- The project is open source under the Apache License 2.0.
- User-facing CPU percentages are rounded to the nearest whole percent; collection and alert evaluation retain the original measurement precision.

### Decisions Deferred

- Concrete source-file structure and dependency selection are determined by `/mdt:architecture`.
- Executable test layout is determined by `/mdt:tests`.
- Implementation sequencing is determined by `/mdt:tasks`.
- Port inspection and project profiles require later CRs.

## 4. Acceptance Criteria

### Functional

- [ ] The app starts and exits cleanly from the macOS menu bar.
- [ ] The overview shows current system CPU, memory, process count, and ranked processes.
- [ ] Users can search and sort processes by name, PID, CPU, and resident memory.
- [ ] Process histories remain separated when macOS reuses a PID.
- [ ] Users can view recent CPU and memory history for a selected process.
- [ ] Sustained CPU alerts show the process, observed value, threshold, and duration.
- [ ] Alert cooldowns and allowlists prevent repeated notifications according to settings.
- [ ] Users can dismiss an alert without deleting its recorded event.
- [ ] Termination requires confirmation and reports success, refusal, disappearance, or permission failure.
- [ ] Users can change sampling, thresholds, retention, notifications, startup, and privacy settings.
- [ ] Users can delete locally retained monitoring history.

### Non-Functional

- [ ] Core monitoring works without administrator privileges on macOS 14+ Apple Silicon.
- [ ] No monitoring data leaves the device during normal operation.
- [ ] Default sampling is 2 seconds, raw retention is 24 hours, and aggregate retention is 7 days.
- [ ] Idle overhead is measured and documented against the target of less than 1% CPU and 100 MB resident memory.
- [ ] CPU normalization is visible in help text and consistent across overview, process detail, alerts, and tests; displayed percentages are rounded to the nearest whole percent.
- [ ] The interface follows the supplied dark visual direction and remains usable with VoiceOver and keyboard navigation.

### Edge Cases

- A process exits between collection and display.
- A PID is reused for a different process.
- Process metadata is inaccessible without elevated privileges.
- The database is unavailable, locked, corrupt, or exceeds retention expectations.
- Sampling is delayed by sleep, wake, system load, or app suspension.
- A termination target disappears or changes identity before confirmation.
- Notifications are denied by macOS.

## 5. Verification

### How to Verify Success

- Manual verification: run the signed-off local build through menu-bar, dashboard, settings, alert, deletion, and termination journeys.
- Automated verification: exercise identity, sampling, aggregation, retention, alert state, settings, bridge contracts, and UI behavior.
- Performance verification: measure idle CPU and resident memory at the default cadence on the supported reference Mac.
- Privacy verification: observe outbound connections during normal operation and confirm no monitoring payload leaves the device.
- Documentation verification: confirm metric semantics, permissions, limitations, retention, and safe termination behavior are documented.

## 6. References

- Ticket concept: [MVP concept and visual contract](./MCS-001/concept.md)
- Requirements trace projection: [requirements.trace.md](./MCS-001/requirements.trace.md)
- Requirements notes: [requirements.md](./MCS-001/requirements.md)
- BDD trace projection: [bdd.trace.md](./MCS-001/bdd.trace.md)
- BDD notes: [bdd.md](./MCS-001/bdd.md)
- Architecture trace projection: [architecture.trace.md](./MCS-001/architecture.trace.md)
- Architecture notes: [architecture.md](./MCS-001/architecture.md)
- Tests trace projection: [tests.trace.md](./MCS-001/tests.trace.md)
- Tasks trace projection: [tasks.trace.md](./MCS-001/tasks.trace.md)
- Implementation tasks: [tasks.md](./MCS-001/tasks.md)
- Source concept: `/Users/ihorchernin/Downloads/MacSentinel_Project_Idea.md`
- Visual reference: supplied MacSentinel dashboard screenshot dated 2026-11-06.

## 7. Workflow

- Next: `/mdt:requirements MCS-001`
- Then: `/mdt:bdd MCS-001` → `/mdt:architecture MCS-001` → `/mdt:tests MCS-001` → `/mdt:tasks MCS-001` → `/mdt:implement MCS-001`
