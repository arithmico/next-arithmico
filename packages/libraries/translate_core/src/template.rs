use std::collections::HashMap;

use crate::error::TranslationError;

#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    items: Vec<TranslationTemmplateItem>,
}

impl Template {
    fn new_with_items(items: Vec<TranslationTemmplateItem>) -> Self {
        Self { items }
    }

    pub fn new<T: ToString>(template: T) -> Self {
        parse_template(&template.to_string()).expect("failed to parse template")
    }

    pub fn render(&self) -> Result<String, TranslationError> {
        let key_map = HashMap::new();
        self.render_with(&key_map)
    }

    pub fn render_with(
        &self,
        key_map: &HashMap<String, String>,
    ) -> Result<String, TranslationError> {
        self.items
            .iter()
            .map(|item| item.render(key_map))
            .collect::<Result<String, _>>()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TranslationTemmplateItem {
    Text(String),
    Key(String),
}

impl TranslationTemmplateItem {
    fn render(
        &self,
        key_map: &HashMap<String, String>,
    ) -> Result<String, TranslationError> {
        match self {
            TranslationTemmplateItem::Text(text) => Ok(text.clone()),
            TranslationTemmplateItem::Key(key) => match key_map.get(key) {
                Some(text) => Ok(text.clone()),
                None => Err(TranslationError::MissingKey(key.clone())),
            },
        }
    }
}

fn parse_template(input: &str) -> Result<Template, String> {
    let mut items = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        if remaining.starts_with("{") {
            let end = remaining
                .find("}")
                .ok_or("closing \"}\" is missing")?;
            let key = &remaining[1..end].trim();

            if key.is_empty() {
                return Err("empty template key".into());
            }
            if key.contains(['{', '}', ' ']) {
                return Err("invalid characters in template key".into());
            }

            items.push(TranslationTemmplateItem::Key(key.to_string()));
            remaining = &remaining[(end + 1)..];
        } else {
            let end = remaining.find('{').unwrap_or(remaining.len());
            let text = &remaining[..end];

            items.push(TranslationTemmplateItem::Text(text.to_string()));
            remaining = &remaining[end..];
        }
    }

    Ok(Template::new_with_items(items))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::template::{Template, TranslationTemmplateItem};

    #[test]
    fn parse_text() {
        assert_eq!(
            Template::new("hello world"),
            Template {
                items: vec![TranslationTemmplateItem::Text(
                    "hello world".into()
                )]
            }
        )
    }

    #[test]
    fn parse_key() {
        assert_eq!(
            Template::new("{my_key}"),
            Template {
                items: vec![TranslationTemmplateItem::Key("my_key".into())]
            }
        )
    }

    #[test]
    fn parse_key_trim_whitespace() {
        assert_eq!(
            Template::new("{  my_key  }"),
            Template {
                items: vec![TranslationTemmplateItem::Key("my_key".into())]
            }
        )
    }

    #[test]
    #[should_panic]
    fn parse_key_failed_due_to_whitespace() {
        Template::new("{my key}");
    }

    #[test]
    fn parse_template_with_key_and_text() {
        assert_eq!(
            Template::new("Hello, my name is {name}!"),
            Template {
                items: vec![
                    TranslationTemmplateItem::Text("Hello, my name is ".into()),
                    TranslationTemmplateItem::Key("name".into()),
                    TranslationTemmplateItem::Text("!".into()),
                ]
            }
        )
    }

    #[test]
    fn render_template_with_key() {
        let template = Template::new("Hello, my name is {name}!");
        let mut keys = HashMap::new();
        keys.insert("name".to_string(), "John Doe".to_string());
        assert_eq!(
            template.render_with(&keys).unwrap(),
            "Hello, my name is John Doe!"
        )
    }

    #[test]
    fn render_template_with_keys() {
        let template =
            Template::new("Hello, my name is {name} and I am {age} years old.");
        let mut keys = HashMap::new();
        keys.insert("name".to_string(), "Jane Doe".to_string());
        keys.insert("age".to_string(), "42".to_string());
        assert_eq!(
            template.render_with(&keys).unwrap(),
            "Hello, my name is Jane Doe and I am 42 years old."
        )
    }

    #[test]
    fn render_template() {
        let template = Template::new("Hello World!");
        assert_eq!(template.render().unwrap(), "Hello World!")
    }
}
