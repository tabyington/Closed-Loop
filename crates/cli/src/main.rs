use clap::Parser;
use chrono::Utc;
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "echo-cli", version, about = "Closed Loop runner (headless)")]
struct Args {
    #[arg(long, default_value_t = 42)]
    seed: u64,

    #[arg(long, default_value_t = 10_000)]
    ticks: u64,
}

#[derive(Serialize)]
struct Manifest {
    seed: u64,
    ticks: u64,
    timestamp: String,
    git_commit: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let cfg = engine::EngineConfig {
        seed: args.seed,
        ticks: args.ticks,
    };

    let summary = engine::Engine::run_headless(&cfg);

    println!("RunSummary: {:?}", summary);

    // === Create run directory ===
    let timestamp = Utc::now().format("%Y-%m-%dT%H-%M-%SZ").to_string();
    let mut run_dir = PathBuf::from("runs");
    run_dir.push(&timestamp);

    fs::create_dir_all(&run_dir)?;

    // === Write manifest.json ===
    let manifest = Manifest {
        seed: args.seed,
        ticks: args.ticks,
        timestamp: timestamp.clone(),
        git_commit: git_commit_hash(),
    };

    let manifest_path = run_dir.join("manifest.json");
    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    fs::write(manifest_path, manifest_json)?;

    // === Write metrics.jsonl (stub) ===
    let metrics_path = run_dir.join("metrics.jsonl");
    let mut file = fs::File::create(metrics_path)?;

    let line = serde_json::to_string(&summary)?;
    writeln!(file, "{}", line)?;

    println!("Run artifacts written to runs/{}", timestamp);

    Ok(())
}

fn git_commit_hash() -> Option<String> {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;

    if !out.status.success() {
        return None;
    }

    let s = String::from_utf8(out.stdout).ok()?;
    Some(s.trim().to_string())
}