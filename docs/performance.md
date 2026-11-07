# Performance Validation

MacSentinel must remain lightweight enough that monitoring does not materially change the system it observes.

## Release budget

On the reference MacBook Pro with Apple M4 Pro and 48 GB RAM, an idle release build must remain below:

- 1% average application CPU
- 100 MB resident memory

These are release gates, not measurements from the current bootstrap.

## Measurement protocol

1. Build a release bundle with production capabilities.
2. Start MacSentinel with default settings and no debug tooling attached.
3. Allow a warm-up period long enough to initialize storage and establish CPU baselines.
4. Measure application CPU and resident memory over a representative idle window.
5. Record the build revision, macOS version, hardware class, sample cadence, window duration, average, and peak values.
6. Repeat after a restart and investigate material variance.

Do not record device serial numbers, hardware UUIDs, user names, file contents, command-line arguments, or other unnecessary private data in benchmark evidence.

## Evidence status

Performance evidence will be recorded during the release-validation task after sampling, storage, and UI behavior exist. Until then, the budget remains unverified.
