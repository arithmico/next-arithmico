use std::collections::HashMap;

use translate_core::TranslatedMessage;

use crate::core::translation_resolver;

use super::{EvaluateNodeError, EvaluateNodeErrorKind};

pub struct GenericRuntimeErrorBuilder {
    id: String,
    keys: HashMap<String, String>,
}

impl GenericRuntimeErrorBuilder {
    pub fn key(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.keys.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(self) -> EvaluateNodeError {
        let mut message = TranslatedMessage::new(self.id, translation_resolver);

        for (key, value) in self.keys {
            message.add_key(key, value);
        }

        EvaluateNodeError::new(EvaluateNodeErrorKind::RuntimeError, message)
    }
}

impl EvaluateNodeError {
    pub fn generic_runtime_error<T: ToString>(
        id: T,
    ) -> GenericRuntimeErrorBuilder {
        GenericRuntimeErrorBuilder {
            id: id.to_string(),
            keys: HashMap::new(),
        }
    }
}
