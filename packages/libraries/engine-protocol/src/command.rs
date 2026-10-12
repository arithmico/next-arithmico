use common::{AngleUnit, Language};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandEnvelope {
    pub correlation_id: Option<usize>,
    pub payload: Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    Evaluate {
        input: String,
        language: Language,
        angle_unit: AngleUnit,
    },
    ClearStack,
}
