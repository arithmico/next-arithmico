use std::collections::HashMap;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{none_of, space0},
    combinator::all_consuming,
    multi::{many0, many1},
    sequence::delimited,
};

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
        let (_, template) = parse_template(&template.to_string())
            .expect("failed to parse template");

        template
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
            .map(|item| item.render(&key_map))
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

fn parse_template(input: &str) -> IResult<&str, Template> {
    let (_, items) =
        all_consuming(many0(alt((parse_template_text, parse_template_key))))
            .parse(input)?;

    Ok(("", Template::new_with_items(items)))
}

fn parse_template_text(input: &str) -> IResult<&str, TranslationTemmplateItem> {
    let (remaining_input, text) = many1(none_of("{}")).parse(input)?;

    Ok((
        remaining_input,
        TranslationTemmplateItem::Text(text.into_iter().collect()),
    ))
}

fn parse_template_key(input: &str) -> IResult<&str, TranslationTemmplateItem> {
    let (remaining_input, text) = delimited(
        (tag("{"), space0),
        many1(none_of("{} ")),
        (space0, tag("}")),
    )
    .parse(input)?;

    Ok((
        remaining_input,
        TranslationTemmplateItem::Key(text.into_iter().collect()),
    ))
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
