use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchWorkflow {
    pub repository_id: u64,
    pub workflow_id: String,
    pub git_ref: String,
}
