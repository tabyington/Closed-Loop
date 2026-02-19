# REQ-0001 — Deterministic run artifacts and snapshot count

## Intent
A headless run must generate reproducible artifacts (manifest + metrics) with predictable snapshot behavior.

## Scope
- In scope:
  - `cli` writes a run directory containing `manifest.json` and `metrics.jsonl`
  - `manifest.json` records run configuration and code identity
  - `metrics.jsonl` contains periodic snapshots of engine state
  - Snapshot count is deterministic and testable
- Out of scope:
  - Real biology, world modeling, Bevy rendering
  - Performance optimization

## Inputs
CLI args:
- `--seed <u64>`
- `--ticks <u64>`
- `--snapshot-interval <u64>`
- `--out-dir <path>`
- `--run-id <string>`

## Acceptance Criteria
- [ ] AC1: Running the CLI produces a run folder at `{out_dir}/{run_id}/` (or `{out_dir}/{timestamp}/` if no run-id) containing:
      - `manifest.json`
      - `metrics.jsonl`
- [ ] AC2: `manifest.json` must include at minimum:
      - seed
      - ticks
      - snapshot_interval
      - run_id (resolved folder name)
      - timestamp
      - git_commit (nullable if git unavailable)
- [ ] AC3: `metrics.jsonl` must contain snapshots at ticks that are multiples of `snapshot_interval`
- [ ] AC4: Snapshot count must equal `floor(ticks / snapshot_interval)` when `snapshot_interval > 0`
      - Example: ticks=5000, interval=1000 → 5 snapshots (1000..5000)
- [ ] AC5: `--snapshot-interval 0` must not crash; it must behave as interval=1

## Metrics / Gates
N/A (this is scaffolding correctness)

## Test Plan
Integration-style tests (in Rust) that:
1. Run the CLI binary with a temporary output directory and `--run-id test-run`
2. Validate required files exist
3. Parse `manifest.json` and validate required fields
4. Read `metrics.jsonl` and validate snapshot count + tick sequence

## Notes
This requirement intentionally creates the harness needed for future CLRD gates and scenario testing.
