use chrono::Utc;
use clap::Parser;
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

    #[arg(long, default_value_t = 1000)]
    snapshot_interval: u64,
}

#[derive(Serialize)]
struct Manifest {
    seed: u64,
    ticks: u64,
    timestamp: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let cfg = engine::EngineConfig {
        seed: args.seed,
        ticks: args.ticks,
    };

    let mut state = engine::Engine::initialize(&cfg);

    // snapshot interval
    let snapshot_interval = args.snapshot_interval.max(1);

       // === Create run directory ===
    let timestamp = Utc::now().format("%Y-%m-%dT%H-%M-%SZ").to_string();
    let mut run_dir = PathBuf::from("runs");
    run_dir.push(&timestamp);

    fs::create_dir_all(&run_dir)?;

    // create metrics file
    let metrics_path = run_dir.join("metrics.jsonl");
    let mut metrics_file = fs::File::create(metrics_path)?;

    for _ in 0..cfg.ticks {
        engine::Engine::step(&mut state);

        if state.current_tick % snapshot_interval == 0 {
            let line = serde_json::to_string(&state)?;
            writeln!(metrics_file, "{}", line)?;
        }
    }

    println!("Final state: {:?}", state);

    // === Write manifest.json ===
    let manifest = Manifest {
        seed: args.seed,
        ticks: args.ticks,
        timestamp: timestamp.clone(),
    };

    let manifest_path = run_dir.join("manifest.json");
    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    fs::write(manifest_path, manifest_json)?;

    println!("Run artifacts written to runs/{}", timestamp);

    Ok(())
}
