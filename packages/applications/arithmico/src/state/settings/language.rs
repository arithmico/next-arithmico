use common::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum LanguageValue {
    System,
    Language(Language),
}
