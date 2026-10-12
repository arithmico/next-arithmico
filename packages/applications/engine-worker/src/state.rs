use engine::load_host_api;
use evaluator::{Api, Stack};

use crate::channel::Channel;

#[allow(dead_code)]
#[derive(Debug)]
pub struct State {
    pub stack: Stack,
    pub host_api: Api,
    pub next_correlation_id: usize,
    pub channel: Channel,
}

impl State {
    pub fn new(channel: Channel) -> Self {
        Self {
            stack: Default::default(),
            host_api: load_host_api(),
            next_correlation_id: 0,
            channel,
        }
    }
}
