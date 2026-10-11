use common::AngleUnit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandEvenlope {
    pub correlation_id: usize,
    pub payload: Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    Evaluate {
        input: String,
        angle_unit: AngleUnit,
    },
    ClearStack,
}
