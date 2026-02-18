fn main() {
    let cfg = engine::EngineConfig { seed: 42, ticks: 10_000 };
    let summary = engine::Engine::run_headless(&cfg);

    println!("RunSummary: {:?}", summary);
}
