use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use tempfile::tempdir;

fn run_cli(
    out_dir: &std::path::Path,
    run_id: &str,
    seed: u64,
    ticks: u64,
    snapshot_interval: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("cli");
    cmd.args([
        "--seed",
        &seed.to_string(),
        "--ticks",
        &ticks.to_string(),
        "--snapshot-interval",
        &snapshot_interval.to_string(),
        "--out-dir",
        out_dir.to_str().unwrap(),
        "--run-id",
        run_id,
    ]);

    cmd.assert().success();
    Ok(())
}



#[test]
fn req_0002_same_inputs_produce_identical_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempdir()?;
    let out = tmp.path();

    let seed = 123;
    let ticks = 5000;
    let interval = 500;

    run_cli(out, "run-a", seed, ticks, interval)?;
    run_cli(out, "run-b", seed, ticks, interval)?;

    let a = fs::read(out.join("run-a").join("metrics.jsonl"))?;
    let b = fs::read(out.join("run-b").join("metrics.jsonl"))?;

    assert_eq!(a, b, "metrics.jsonl differs for identical inputs");

    Ok(())
}

#[test]
fn req_0002_different_seed_produces_different_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempdir()?;
    let out = tmp.path();

    let ticks = 5000;
    let interval = 500;

    run_cli(out, "seed-1", 123, ticks, interval)?;
    run_cli(out, "seed-2", 124, ticks, interval)?;

    let a = fs::read_to_string(out.join("seed-1").join("metrics.jsonl"))?;
    let b = fs::read_to_string(out.join("seed-2").join("metrics.jsonl"))?;

    assert_ne!(a, b, "metrics.jsonl unexpectedly identical for different seeds");

    Ok(())
}
