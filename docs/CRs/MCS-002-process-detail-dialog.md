---
code: MCS-002
status: Implemented
dateCreated: 2026-11-07T17:20:24.328Z
type: Bug Fix
priority: Medium
---

# Show process details without scrolling

## 1. Description

### Requirements Scope

`brief`

### Problem

- `src/components/ProcessExplorer.tsx` appends the selected process detail after the complete process table.
- Long process lists force users to scroll to the bottom before reading details or reaching the Close control.
- `src/styles.css` provides no viewport-level detail presentation or background interaction boundary.

### Affected Artifacts

- `src/components/ProcessExplorer.tsx` (selection and detail presentation)
- `src/styles.css` (dialog, backdrop, responsive layout, and focus styling)
- `src/App.test.tsx` (accessibility and interaction regression coverage)
- `e2e/macsentinel.e2e.ts` (integrated process-detail journey)

### Scope

- Changes: render selected process details in an immediately visible modal dialog with a backdrop and close controls.
- Changes: support Close button, Escape, and backdrop dismissal while preserving keyboard activation of process rows.
- Unchanged: process collection, sorting, searching, identity, metrics, and termination behavior.

## 2. Decision

### Chosen Approach

- Render process details as a viewport-fixed accessible modal dialog owned by `ProcessExplorer`.

### Rationale

- The dialog is visible immediately regardless of table length or scroll position.
- Existing `selected` state and process data remain the single presentation source.
- A modal boundary prevents the long table from competing with the selected detail.
- Escape and explicit Close behavior provide keyboard-accessible dismissal.

## 3. Alternatives Considered

| Approach | Key Difference | Why Rejected |
|----------|---------------|--------------|
| Viewport-fixed modal dialog | Immediate detail presentation above the table | **ACCEPTED** - Works at every table length and viewport width |
| Sticky side panel | Keeps table and details visible together | Reduces table width and performs poorly on narrow windows |
| Scroll selected detail into view | Retains appended layout | Still moves users away from their selected row and list context |

## 4. Artifact Specifications

### New Artifacts

| Artifact | Type | Purpose |
|----------|------|---------|
| None | — | Existing component and test owners are sufficient |

### Modified Artifacts

| Artifact | Change Type | Modification |
|----------|-------------|--------------|
| `src/components/ProcessExplorer.tsx` | Presentation fix | Render selected detail through an accessible modal layer |
| `src/styles.css` | Layout fix | Add fixed backdrop and responsive dialog styles |
| `src/App.test.tsx` | Regression coverage | Verify immediate dialog visibility and dismissal |
| `e2e/macsentinel.e2e.ts` | Journey update | Verify the selected detail is presented as a dialog |

### Integration Points

| From | To | Interface |
|------|----|-----------|
| Process table row | Process detail dialog | `selected: ProcessSnapshot | null` |
| Dialog controls | Process explorer state | `setSelected(null)` |

### Key Patterns

- Accessible modal dialog: `role="dialog"`, `aria-modal="true"`, and labelled title.
- Presentation ownership: `ProcessExplorer` remains the only owner of selected-process UI state.

## 5. Acceptance Criteria

### Functional

- [x] Selecting any process opens its details within the current viewport without scrolling to the table end.
- [x] The dialog shows name, PID, start time, parent PID, CPU, memory, and executable path.
- [x] Close button, Escape, and backdrop interaction dismiss the dialog.
- [x] Keyboard activation with Enter or Space continues to open process details.

### Non-Functional

- [x] The dialog exposes an accessible name and modal semantics.
- [x] The dialog remains usable in narrow application windows.
- [x] Process collection, filtering, sorting, and identity behavior remain unchanged.

### Testing

- Unit: select a process row and verify `role="dialog"` is immediately present.
- Unit: press Escape and verify the process dialog is removed.
- Integration: open a live process and verify the modal detail is displayed.
- Build: run the TypeScript and Vite production build.

## 6. Verification

### By CR Type

- Bug Fix: process detail no longer renders after the full table in the regression test and live journey.

### Verifiable Artifacts

- `src/App.test.tsx` passes the modal visibility and dismissal cases.
- `e2e/macsentinel.e2e.ts` identifies the process detail by dialog semantics.
- `bun run build` completes successfully.

## 7. Deployment

### Simple Changes

- Ship with the next MacSentinel application build.
- No configuration or data migration is required.
- Roll back by reverting the component, styles, and tests in this CR.
