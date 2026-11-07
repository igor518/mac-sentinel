# Tasks: MCS-002

## Task 1: Move process detail into accessible viewport dialog

**Canonical Task**: `TASK-1`

**Scope**: Replace the appended process-detail block with a fixed modal presentation owned by `ProcessExplorer`.

**Boundary**: Do not change process collection, identity, search, sorting, metrics, storage, alerts, or termination.

**Modifies**:

- `src/components/ProcessExplorer.tsx`
- `src/styles.css`
- `src/App.test.tsx`
- `e2e/macsentinel.e2e.ts`

**Verify**:

```bash
bun run test
bun run build
```

**Done when**:

- [x] Selecting a process opens an accessible dialog in the current viewport.
- [x] The dialog closes through Close, Escape, and backdrop interaction.
- [x] The dialog remains usable in narrow windows and exposes visible focus.
- [x] Frontend tests and the production build pass.
