use crate::config::StrategyConfig;
use anyhow::Result;
use std::time::{Duration, Instant};

pub struct EventDetector {
    strategy: StrategyConfig,
    last_tick: Instant,
}

impl EventDetector {
    pub fn new(strategy: StrategyConfig) -> Self {
        Self {
            strategy,
            last_tick: Instant::now(),
        }
    }

    pub async fn event_detected(&self) -> Result<bool> {
        let elapsed = self.last_tick.elapsed();
        Ok(elapsed >= Duration::from_secs(self.strategy.check_interval_secs))
    }
}
