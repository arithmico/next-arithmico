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
        parse_template(&template.to_string())
            .unwrap_or_else(|_| Self::new_with_items(vec![]))
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
    let mut buffer = String::new();
    let mut chars = input.chars();
    let mut is_key = false;

    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some('{') => buffer.push('{'),
                Some('}') => buffer.push('}'),
                other => {
                    buffer.push(c);
                    if let Some(other) = other {
                        buffer.push(other);
                    }
                }
            },
            '{' => {
                if is_key {
                    return Err(String::from(
                        "SyntaxError: nested '{' are not allowed",
                    ));
                }
                if !buffer.is_empty() {
                    items.push(TranslationTemmplateItem::Text(buffer.clone()));
                    buffer.clear();
                }
                is_key = true;
            }
            '}' => {
                if !is_key {
                    return Err(String::from(
                        "SyntaxError: missing opening brace",
                    ));
                }
                if buffer.is_empty() {
                    return Err(String::from("SyntaxError: missing key name"));
                }
                items.push(TranslationTemmplateItem::Key(buffer.clone()));
                buffer.clear();
                is_key = false;
            }
            c => {
                if c.is_whitespace() && is_key {
                    continue;
                }
                buffer.push(c)
            }
        }
    }
    if is_key {
        return Err(String::from("SyntaxError: missing closing brace"));
    }
    if !buffer.is_empty() {
        items.push(TranslationTemmplateItem::Text(buffer.clone()));
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
    fn parse_text_with_excaped_braces() {
        assert_eq!(
            Template::new("\\{hello world\\}"),
            Template {
                items: vec![TranslationTemmplateItem::Text(
                    "{hello world}".into()
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
