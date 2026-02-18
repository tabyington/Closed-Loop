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

#[derive(Debug, Clone, Serialize)]
pub struct EngineState {
    pub seed: u64,
    pub current_tick: u64,
    pub births: u64,
    pub deaths: u64,
}

#[derive(Serialize)]
pub struct Engine;

impl Engine {
    pub fn initialize(cfg: &EngineConfig) -> EngineState {
        EngineState {
            seed: cfg.seed,
            current_tick: 0,
            births: 0,
            deaths: 0,
        }
    }

    pub fn step(state: &mut EngineState) {
        state.current_tick += 1;

        // Deterministic placeholder logic
        if state.current_tick % 1000 == 0 {
            state.births += state.seed % 10;
        }

        if state.current_tick % 2500 == 0 {
            state.deaths += (state.seed / 10) % 5;
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
