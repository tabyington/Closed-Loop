use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "echo-cli", version, about = "Closed Loop runner (headless)")]
struct Args {
    /// RNG seed for deterministic runs
    #[arg(long, default_value_t = 42)]
    seed: u64,

    /// Number of simulation ticks
    #[arg(long, default_value_t = 10_000)]
    ticks: u64,
}

fn main() {
    let args = Args::parse();

    let cfg = engine::EngineConfig {
        seed: args.seed,
        ticks: args.ticks,
    };
    
    let summary = engine::Engine::run_headless(&cfg);

    println!("RunSummary: {:?}", summary);
}