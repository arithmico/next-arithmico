use engine_derive::FunctionArguments;
use evaluator::Error;
use node::Number;

use crate::{
    Context,
    core::{FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("min")]
#[description(
    Language::German,
    "Gibt den kleinsten Wert aus den uebergebenen Werten zurück."
)]
#[description(
    Language::English,
    "Returns the smallest value among the given arguments."
)]
pub struct MinArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: Vec<&'a Number>,
}

pub struct MinEndpoint;

impl FunctionEndpoint for MinEndpoint {
    type Output = Number;

    type Arguments<'a> = MinArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        MinArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, Error> {
        let value = match &x[..] {
            [] => return Err(Error::missing_parameter("x")),
            [item] => item.value,
            [item, rest @ ..] => {
                rest.iter().fold(item.value, |acc, v| acc.min(v.value))
            }
        };

        Ok(Number::new(value))
    }
}
