# Requirements: MCS-001

**Source**: [MCS-001](../MCS-001-process-monitor.md)
**Generated**: 2026-11-07

## Overview

- MacSentinel provides local, evidence-based macOS process monitoring for developers.
- The MVP progresses from a read-only collector and live overview through history, settings, alerting, and confirmation-gated termination.
- The product prioritizes least privilege, explicit measurement semantics, bounded resource use, and clear degraded states.

## Delivery Slices

| Order | Slice | Completion Signal |
|------:|-------|-------------------|
| 1 | Collector, process identity, dashboard, ranked processes, sustained CPU alert | Current metrics and one CPU alert journey operate end to end. |
| 2 | Short-term history and bounded SQLite persistence | Process history survives restart and obeys retention. |
| 3 | Settings, notifications, cooldowns, allowlists, deletion | Saved settings alter subsequent monitoring behavior. |
| 4 | Experimentally validated memory-growth alert | Recorded workload evidence supports the selected heuristic. |
| 5 | Confirmation-gated termination | Identity is revalidated before any signal is sent. |

## Constraint Carryover

| Constraint IDs | Must Appear In |
|----------------|----------------|
| C1, C2 | Architecture privilege and network boundaries; integration and privacy tests; every implementation task touching platform access. |
| C3, C4, Edge-4 | Collector timing and normalization design; boundary tests; alert tasks. |
| C5, C8, Edge-3 | Storage architecture, pruning tests, degraded-mode tasks. |
| C6 | Benchmark harness, performance verification, release evidence. |
| C7, Edge-1, Edge-2, Edge-5 | Process identity owner, lifecycle tests, alert and termination tasks. |
| C9 | Design tokens, component tests, accessibility verification. |
| C10 | Tauri configuration, repository license, distribution notices. |

## Non-Ambiguity Table

| Concept | Final Semantic | Rejected Semantic | Why |
|---------|----------------|-------------------|-----|
| System CPU | `100%` means all logical cores fully utilized. | `100%` per logical core. | Matches whole-machine overview expectations. |
| Process CPU | `100%` means one logical core; a process may exceed `100%`. | Clamp every process to `100%`. | Preserves multi-threaded workload evidence. |
| CPU display precision | Round user-facing percentages to the nearest whole percent while retaining raw measurement precision internally. | Round measurements before alert evaluation or persistence. | Keeps the interface readable without changing evidence semantics. |
| Process identity | PID plus process start time. | PID alone or executable path alone. | Prevents PID reuse from merging unrelated history or actions. |
| Sustained duration | Only consecutive valid samples count; gaps reset continuity. | Treat sleep or delayed sampling as continuous evidence. | Avoids false sustained alerts. |
| Missing data | Present the field as unavailable and preserve known fields. | Substitute zero or discard the process. | Zero is a measurement and would be misleading. |
| Termination | Revalidate identity after confirmation, then send the safe default signal. | Signal by PID without revalidation or terminate automatically. | Avoids acting on a reused PID or unintended process. |
| Alert language | State observations, thresholds, and durations. | Assert an unverified root cause. | Maintains evidence-based diagnostics. |
| Storage failure | Continue read-only live monitoring with a visible degraded state. | Stop the app or silently drop persistence. | Preserves the primary monitoring function transparently. |

## Configuration

| Setting | Default | Validation | When Unavailable |
|---------|---------|------------|------------------|
| Sampling interval | 2 seconds | Positive supported cadence. | Use the documented default. |
| CPU threshold | 80% of one logical core | Non-negative process-CPU percentage. | Use the documented default. |
| CPU sustained duration | 10 minutes | Positive duration longer than one sample. | Use the documented default. |
| Raw retention | 24 hours | Non-negative bounded duration. | Use the documented default. |
| Aggregate retention | 7 days | Not shorter than raw retention. | Use the documented default. |
| Notifications | User controlled | Reflect macOS authorization state. | Keep alerts visible in-app. |
| Start at login | Off until user enables it | Reflect plugin and OS capability. | Continue manual launch. |

## Open Architecture Work

- Select the collector library and validate macOS metric behavior.
- Select the SQLite client and migration strategy.
- Define the typed Rust-to-frontend snapshot and command contracts.
- Define the bounded queue, pruning, and degraded-storage behavior.
- Choose the executable UI acceptance framework for a Tauri application.
- Design a repeatable CPU and memory benchmark on the reference Mac.
- Define the memory-growth experiment before setting a default rule.

---

Use `requirements.trace.md` for canonical requirement rows and route summaries.
