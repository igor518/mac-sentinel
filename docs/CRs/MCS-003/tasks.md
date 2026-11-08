# Tasks: MCS-003

## Task 1: Bound process scrolling and add metric trends

**Canonical Task**: `TASK-1`

**Scope**: Keep the system overview visible, make the process table the dashboard scroll owner, and add bounded accessible CPU and memory trends.

**Boundary**: Do not change Rust sampling, persistence, alert evaluation, process identity, search, sorting, or process-detail behavior.

**Creates**:

- `src/components/TrendChart.tsx`

**Modifies**:

- `src/components/Dashboard.tsx`
- `src/components/ProcessExplorer.tsx`
- `src/styles.css`
- `src/App.test.tsx`
- `e2e/mcs003.e2e.ts`

**Verify**:

```bash
bun run test
bun run build
bun run test:e2e -- --spec e2e/mcs003.e2e.ts
```

**Done when**:

- [x] System metrics remain visible while the process table scrolls.
- [x] Process heading and controls remain visible above the scroll region.
- [x] CPU and memory cards show accessible trends from at most 60 available samples.
- [x] Missing samples do not become fabricated zero values.
- [x] Frontend tests and the production build pass.
