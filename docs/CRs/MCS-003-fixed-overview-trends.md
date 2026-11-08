---
code: MCS-003
status: Implemented
dateCreated: 2026-11-08T00:02:59.144Z
type: Feature Enhancement
priority: Medium
---

# Keep overview fixed and add metric trends

## 1. Description

### Requirements Scope

`full`

### Problem

- `src/styles.css` lets the complete dashboard grow beyond the native window, so users lose current system metrics while scrolling a long process table.
- `src/components/ProcessExplorer.tsx` provides horizontal overflow but no bounded vertical process region.
- `src/components/Dashboard.tsx` shows only the latest CPU and memory values, so short-term movement is not visible.

### Affected Artifacts

- `src/components/Dashboard.tsx` (overview layout and live trend history)
- `src/components/ProcessExplorer.tsx` (bounded process scroll region)
- `src/styles.css` (viewport-height dashboard and overflow ownership)
- `src/App.test.tsx` (trend and scroll-region regression coverage)
- `e2e/mcs003.e2e.ts` (native scroll ownership verification)

### Scope

- Changes: keep the header, notices, system metrics, semantics, and alerts visible within the native window.
- Changes: make the process table the only vertically scrolling dashboard region while keeping its heading and controls visible.
- Changes: add accessible CPU and memory SVG sparklines from the latest 60 live snapshots.
- Unchanged: Rust sampling, metric precision, alert evaluation, persistence, process identity, sorting, searching, and process details.

## 2. Decision

### Chosen Approach

- Use a viewport-bounded CSS layout and lightweight React-owned SVG sparklines.

### Rationale

- The process table can contain hundreds of rows while the system overview remains available.
- Sixty snapshots provide approximately two minutes of context at the current two-second cadence.
- SVG sparklines add no runtime dependency and consume the existing immutable snapshot stream.
- Raw values remain unchanged; rounding applies only to labels and chart accessibility text.

## 3. Alternatives Considered

| Approach | Key Difference | Why Rejected |
|----------|---------------|--------------|
| CSS-bounded process region with SVG sparklines | Keeps overview fixed and renders local trends | **ACCEPTED** - Uses existing data and adds no dependency |
| Sticky overview in a document-scrolling page | Retains full-page scrolling | Sticky sections compete for space and can overlap alerts |
| Charting library with full axes and interaction | Adds tooltips and chart primitives | Excess dependency and visual weight for compact live context |

## 4. Artifact Specifications

### New Artifacts

| Artifact | Type | Purpose |
|----------|------|---------|
| `src/components/TrendChart.tsx` | Component | Render an accessible compact SVG trend from recent numeric samples |

### Modified Artifacts

| Artifact | Change Type | Modification |
|----------|-------------|--------------|
| `src/components/Dashboard.tsx` | Presentation enhancement | Retain 60 system samples and render CPU and memory trends |
| `src/components/ProcessExplorer.tsx` | Layout contract | Mark the process table wrapper as the bounded vertical scroll owner |
| `src/styles.css` | Layout enhancement | Bound dashboard height and transfer vertical overflow to the process table |
| `src/App.test.tsx` | Regression coverage | Verify metric trends and the dedicated process scroll region |
| `e2e/mcs003.e2e.ts` | Integrated coverage | Verify the overview position remains fixed while the process table scrolls |

### Integration Points

| From | To | Interface |
|------|----|-----------|
| `Dashboard` snapshot stream | `TrendChart` | Numeric arrays derived from `MonitoringSnapshot` |
| `ProcessExplorer` | `styles.css` | `.process-explorer` and `.table-wrap` overflow contract |

### Key Patterns

- Bounded recent history: retain at most 60 points in React component state.
- Accessible visualization: each SVG exposes a descriptive label and textual range.
- Overflow ownership: the application shell and overview remain fixed; `.table-wrap` owns vertical scrolling.

## 5. Acceptance Criteria

### Functional

- [x] System statistics remain visible while the process list is scrolled.
- [x] The process heading, search, and sort controls remain visible above the scrolling table.
- [x] CPU and memory cards show live trend graphs built from up to 60 snapshots.
- [x] Empty or unavailable metric samples do not fabricate zero values.

### Non-Functional

- [x] Trend history is bounded to 60 points.
- [x] Trend graphs have accessible names and do not rely on color alone.
- [x] Existing process search, sort, detail, storage, and alert behavior remains unchanged.

### Testing

- Unit: render multiple snapshots and verify CPU and memory trend graphs update.
- Unit: verify the process table wrapper exposes the dedicated scroll-region test contract.
- E2E: scroll the native process region and verify the overview screen position is unchanged.
- Build: run TypeScript and Vite production build.
- Manual: scroll a long live process list and verify overview metrics remain stationary.

## 6. Verification

### By CR Type

- Feature: `TrendChart.tsx` exists, frontend tests pass, and only the process table owns dashboard vertical scrolling.

### Verifiable Artifacts

- `src/App.test.tsx` verifies two accessible metric trend graphs.
- `src/components/ProcessExplorer.tsx` identifies the process scroll region.
- `e2e/mcs003.e2e.ts` verifies native scroll ownership.
- `bun run test` and `bun run build` complete successfully.

## 7. Deployment

### Simple Changes

- Ship with the next MacSentinel application build.
- No configuration, database migration, or new dependency is required.
- Roll back by reverting the new chart component and layout styles.
