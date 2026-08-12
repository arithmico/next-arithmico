use std::collections::HashMap;

use common::Language;
use toml::{Table, Value};

use crate::TranslationTemplate;

#[derive(Debug, Clone, PartialEq)]
pub struct TranslationTemplateProvider {
    templates: HashMap<String, TranslationTemplate>,
}

impl TranslationTemplateProvider {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    fn add_translation(
        &mut self,
        id: impl ToString,
        language: Language,
        template: impl ToString,
    ) {
        let id = id.to_string();
        if !self.templates.contains_key(&id) {
            self.templates
                .insert(id.clone(), TranslationTemplate::new());
        }
        let translation_template = self.templates.get_mut(&id).unwrap();
        translation_template.add_translation(language, template);
    }

    fn write_toml_data(&mut self, prefix: &str, table: &Table) {
        let text_entries = table.iter().filter(|(_, value)| match value {
            Value::String(_) => true,
            _ => false,
        });

        let table_entries = table.iter().filter(|(_, value)| match value {
            Value::Table(_) => true,
            _ => false,
        });

        for (key, value) in text_entries {
            if let Ok(language) = key.parse::<Language>() {
                if let Value::String(value) = value {
                    self.add_translation(prefix, language, value);
                }
            }
        }

        for (key, value) in table_entries {
            if let Value::Table(table) = value {
                self.write_toml_data(&format!("{prefix}.{key}"), table);
            }
        }
    }

    fn load_data_from_toml(&mut self, toml: &Table) {
        for (key, value) in toml {
            if let Value::Table(table) = value {
                self.write_toml_data(key, table);
            }
        }
    }

    pub fn try_from_toml(toml: &str) -> Option<Self> {
        let mut provider = Self::new();
        let toml: Table = toml.parse().ok()?;
        provider.load_data_from_toml(&toml);
        Some(provider)
    }

    pub fn get_template(
        &self,
        id: impl ToString,
    ) -> Option<&TranslationTemplate> {
        self.templates.get(&id.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let content = r#"
            [test.foo]
            German = "german foo"
            English = "english foo"

            [test]
            German = "Test"
            English = "test"
        "#;

        let parsed_provider =
            TranslationTemplateProvider::try_from_toml(&content).unwrap();

        let mut expected_provider = TranslationTemplateProvider::new();
        expected_provider.add_translation(
            "test.foo",
            Language::German,
            "german foo",
        );
        expected_provider.add_translation(
            "test.foo",
            Language::English,
            "english foo",
        );
        expected_provider.add_translation("test", Language::German, "Test");
        expected_provider.add_translation("test", Language::English, "test");
        assert_eq!(parsed_provider, expected_provider);
    }
}
