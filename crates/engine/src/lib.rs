use serde::Serialize;

pub fn hello_engine() -> &'static str {
    "Engine Online"
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug, Clone, Serialize)]
pub struct EngineConfig {
    pub seed: u64,
    pub ticks: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunSummary {
    pub seed: u64,
    pub ticks: u64,
    pub births: u64,
    pub deaths: u64,
}

pub struct Engine;

impl Engine {
    pub fn run_headless(cfg: &EngineConfig) -> RunSummary {
        // Placeholder deterministic behavior.
        // We’ll replace this with real sim state soon.
        // Determinism rule: outputs must be a pure function of cfg + code version.

        // Tiny “toy” dynamics just to prove the loop shape.
        let births = cfg.seed % 97; // deterministic
        let deaths = (cfg.seed / 97) % 53;

        RunSummary {
            seed: cfg.seed,
            ticks: cfg.ticks,
            births,
            deaths,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
