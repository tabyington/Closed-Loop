# REQ-0002 — Deterministic run equivalence

## Intent
Runs with identical inputs and code version must produce identical artifacts.

## Scope
- In scope:
  - Same args (seed/ticks/snapshot_interval) + same code ⇒ identical `metrics.jsonl` content
  - Different seed ⇒ `metrics.jsonl` differs in at least one line
- Out of scope:
  - Performance benchmarking
  - Floating-point determinism across platforms (for now)

## Inputs
CLI args:
- `--seed <u64>`
- `--ticks <u64>`
- `--snapshot-interval <u64>`
- `--out-dir <path>`
- `--run-id <string>`

## Acceptance Criteria
- [ ] AC1: Two runs with the same args and different `--run-id` values must produce byte-identical `metrics.jsonl` files.
- [ ] AC2: Two runs with identical args except different `--seed` must produce different `metrics.jsonl` content.

## Test Plan
Integration tests in `crates/cli/tests/req_0002_determinism.rs` that:
- Run the CLI twice with same seed/ticks/interval into two run folders and compare `metrics.jsonl` bytes
- Run again with a different seed and assert `metrics.jsonl` differs
