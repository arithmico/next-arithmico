use engine::{DocumentationModule, SessionError};
use node::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub correlation_id: Option<usize>,
    pub payload: Event,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Ready,
    EvaluationComplete { output: Result<Node, SessionError> },
    StackItemUpdated { name: String, value: Node },
    StackCleared,
    DocumentatonLoaded { modules: Vec<DocumentationModule> },
}
