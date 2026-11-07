# MacSentinel

MacSentinel is a privacy-first macOS process monitor. It is designed to explain current CPU and memory pressure, preserve useful local evidence, and surface sustained resource anomalies without sending monitoring data off the Mac.

The project is currently being built incrementally from the specifications in `docs/CRs/MCS-001`. The bootstrap is intentionally behavior-free; live monitoring is introduced only after the native and frontend boundaries compile and their tests are in place.

## Supported environment

- macOS 14 or later on Apple Silicon
- Intel Macs on a best-effort basis
- Bun for the frontend toolchain
- Rust and Cargo for the native application

## Development

Install dependencies:

```bash
bun install
```

Run frontend tests:

```bash
bun run test
```

Run native tests:

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

The product does not require administrator privileges. Production capabilities must not grant general network or shell execution access.

## Documentation

- Product and delivery artifacts: `docs/CRs/MCS-001`
- Metric definitions: `docs/metric-semantics.md`
- Performance validation: `docs/performance.md`

## License

Licensed under the Apache License, Version 2.0. See `LICENSE`.
