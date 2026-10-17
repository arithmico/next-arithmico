use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use node::Number;

#[derive(FunctionArguments)]
#[name("count")]
#[description(
    Language::German,
    "Gibt die Anzahl der angegebenen Elemente zurück."
)]
#[description(Language::English, "Returns the number of given elements.")]
pub struct CountArgs<'a> {
    #[description(Language::German, "Werte")]
    #[description(Language::English, "values")]
    x: Vec<&'a Number>,
}

pub struct CountEndpoint;

impl FunctionEndpoint for CountEndpoint {
    type Output = Number;
    type Arguments<'a> = CountArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        CountArgs { x }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        Ok(Number::new(x.len() as f64))
    }
}
