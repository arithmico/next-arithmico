use std::{collections::HashMap, fmt::Debug, rc::Rc};

use common::Language;

use crate::{Translatable, TranslationError, TranslationTemplateProvider};

#[derive(Debug, Clone)]
pub struct TranslatedMessage {
    template_id: String,
    keys: HashMap<String, Value>,
    provider_resolver: fn() -> Rc<TranslationTemplateProvider>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Text(String),
    Number(f64),
    Message(Box<TranslatedMessage>),
}

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for &str {
    fn into_value(self) -> Value {
        Value::Text(self.to_string())
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::Text(self)
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Number(self)
    }
}

impl IntoValue for usize {
    fn into_value(self) -> Value {
        Value::Number(self as f64)
    }
}

impl IntoValue for TranslatedMessage {
    fn into_value(self) -> Value {
        Value::Message(Box::new(self))
    }
}

impl<T: IntoValue + Clone> IntoValue for &T {
    fn into_value(self) -> Value {
        self.clone().into_value()
    }
}

impl PartialEq for TranslatedMessage {
    fn eq(&self, other: &Self) -> bool {
        self.template_id == other.template_id && self.keys == other.keys
    }
}

impl TranslatedMessage {
    pub fn new(
        id: impl ToString,
        resolver: fn() -> Rc<TranslationTemplateProvider>,
    ) -> Self {
        Self {
            template_id: id.to_string(),
            keys: HashMap::new(),
            provider_resolver: resolver,
        }
    }

    pub fn add_key(&mut self, key: impl ToString, value: impl IntoValue) {
        self.keys.insert(key.to_string(), value.into_value());
    }

    pub fn key(mut self, key: impl ToString, value: impl IntoValue) -> Self {
        self.add_key(key, value);
        self
    }
}

impl Translatable for TranslatedMessage {
    fn translate(
        &self,
        language: Language,
    ) -> Result<String, TranslationError> {
        let provider = (self.provider_resolver)();
        let template = provider
            .get_template(&self.template_id)
            .ok_or(TranslationError::MissingTranslation(language))?;
        let mut keys = HashMap::new();
        for (key, value) in &self.keys {
            let value = match value {
                Value::Text(text) => text.clone(),
                Value::Number(number) => number.translate(language)?,
                Value::Message(message) => message.translate(language)?,
            };
            keys.insert(key.clone(), value);
        }
        template.translate_with(language, &keys)
    }
}
