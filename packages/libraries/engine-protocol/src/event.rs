use engine::{Documentation, SessionError};
use node::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub correlation_id: Option<usize>,
    pub payload: Event,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Ready { documentation: Documentation },
    EvaluationComplete { output: Result<Node, SessionError> },
    StackItemUpdated { name: String, value: Node },
    StackCleared,
}
