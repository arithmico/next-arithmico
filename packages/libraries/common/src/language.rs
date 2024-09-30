use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum Language {
    German,
    English,
}
