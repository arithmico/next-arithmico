use engine::load_host_api;
use evaluator::{Api, Stack};

#[allow(dead_code)]
#[derive(Debug)]
pub struct State {
    pub stack: Stack,
    pub host_api: Api,
    pub next_correlation_id: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            stack: Default::default(),
            host_api: load_host_api(),
            next_correlation_id: 0,
        }
    }
}
