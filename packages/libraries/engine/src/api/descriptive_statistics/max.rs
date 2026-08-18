use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use node::Number;

#[derive(FunctionArguments)]
#[name("max")]
#[description(
    Language::German,
    "Gibt den größten Wert aus den übergebenen Werten zurück."
)]
#[description(
    Language::English,
    "Returns the largest value among the given arguments."
)]
pub struct MaxArgs<'a> {
    #[description(Language::German, "Werte")]
    #[description(Language::English, "values")]
    x: Vec<&'a Number>,
}

pub struct MaxEndpoint;

impl FunctionEndpoint for MaxEndpoint {
    type Output = Number;
    type Arguments<'a> = MaxArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        MaxArgs { x }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        let value = match &x[..] {
            [] => return Err(Error::missing_parameter("x")),
            [item] => item.value,
            [item, rest @ ..] => {
                rest.iter().fold(item.value, |acc, v| acc.max(v.value))
            }
        };

        Ok(Number::new(value))
    }
}
