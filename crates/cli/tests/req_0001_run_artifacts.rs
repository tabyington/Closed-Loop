use assert_cmd::prelude::*;
use serde_json::Value;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn req_0001_run_artifacts_and_snapshot_count() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let tmp = tempdir()?;
    let out_dir = tmp.path();

    let run_id = "test-run";
    let seed = "7";
    let ticks = "5000";
    let snapshot_interval = "1000";

    // Act: run the CLI binary
    let mut cmd = Command::cargo_bin("cli")?;
    cmd.args([
        "--seed",
        seed,
        "--ticks",
        ticks,
        "--snapshot-interval",
        snapshot_interval,
        "--out-dir",
        out_dir.to_str().unwrap(),
        "--run-id",
        run_id,
    ]);

    cmd.assert().success();

    // Assert: expected files exist
    let run_path = out_dir.join(run_id);
    let manifest_path = run_path.join("manifest.json");
    let metrics_path = run_path.join("metrics.jsonl");

    assert!(
        manifest_path.exists(),
        "manifest.json missing at {}",
        manifest_path.display()
    );
    assert!(
        metrics_path.exists(),
        "metrics.jsonl missing at {}",
        metrics_path.display()
    );

    // Assert: manifest contains required fields
    let manifest_text = fs::read_to_string(&manifest_path)?;
    let manifest: Value = serde_json::from_str(&manifest_text)?;

    for key in [
        "seed",
        "ticks",
        "snapshot_interval",
        "run_id",
        "timestamp",
        "git_commit",
    ] {
        assert!(
            manifest.get(key).is_some(),
            "manifest.json missing key `{}`: {}",
            key,
            manifest_text
        );
    }

    assert_eq!(manifest["seed"].as_u64(), Some(7));
    assert_eq!(manifest["ticks"].as_u64(), Some(5000));
    assert_eq!(manifest["snapshot_interval"].as_u64(), Some(1000));
    assert_eq!(manifest["run_id"].as_str(), Some(run_id));

    // Assert: metrics has exactly floor(ticks / interval) snapshots
    let metrics_text = fs::read_to_string(&metrics_path)?;
    let lines: Vec<&str> = metrics_text.lines().filter(|l| !l.trim().is_empty()).collect();

    let ticks_u = 5000u64;
    let interval_u = 1000u64;
    let expected = (ticks_u / interval_u) as usize;

    assert_eq!(
        lines.len(),
        expected,
        "expected {} snapshots, got {}.\nmetrics:\n{}",
        expected,
        lines.len(),
        metrics_text
    );

    // Assert: snapshots occur at multiples of snapshot_interval (and are monotonic)
    let mut last_tick = 0u64;
    for (i, line) in lines.iter().enumerate() {
        let v: Value = serde_json::from_str(line)?;
        let t = v["current_tick"]
            .as_u64()
            .ok_or("metrics line missing current_tick")?;

        assert_eq!(
            t % interval_u,
            0,
            "line {} tick {} not multiple of {}",
            i,
            t,
            interval_u
        );
        assert!(
            t > last_tick,
            "ticks not strictly increasing ({} then {})",
            last_tick,
            t
        );
        last_tick = t;
    }

    Ok(())
}

#[test]
fn req_0001_snapshot_interval_zero_does_not_crash() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let tmp = tempdir()?;
    let out_dir = tmp.path();
    let run_id = "interval-zero";

    // Act
    let mut cmd = Command::cargo_bin("cli")?;
    cmd.args([
        "--ticks",
        "10",
        "--snapshot-interval",
        "0",
        "--out-dir",
        out_dir.to_str().unwrap(),
        "--run-id",
        run_id,
    ]);

    cmd.assert().success();

    // Assert: should produce 10 snapshots (interval clamped to 1)
    let metrics_path = out_dir.join(run_id).join("metrics.jsonl");
    let metrics_text = fs::read_to_string(&metrics_path)?;
    let lines: Vec<&str> = metrics_text.lines().filter(|l| !l.trim().is_empty()).collect();

    assert_eq!(
        lines.len(),
        10,
        "expected 10 snapshots when interval=0 (clamped to 1), got {}.\nmetrics:\n{}",
        lines.len(),
        metrics_text
    );

    Ok(())
}
