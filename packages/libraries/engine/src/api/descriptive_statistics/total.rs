use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use node::Number;

#[derive(FunctionArguments)]
#[name("total")]
#[description(Language::German, "Gibt die Summe aller Elemente zurück.")]
#[description(Language::English, "Returns the sum of all elements.")]
pub struct TotalArgs<'a> {
    #[description(Language::German, "Werte")]
    #[description(Language::English, "values")]
    x: Vec<&'a Number>,
}

pub struct TotalEndpoint;

impl FunctionEndpoint for TotalEndpoint {
    type Output = Number;
    type Arguments<'a> = TotalArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        TotalArgs { x }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        Ok(Number::new(x.iter().map(|number| number.value).sum()))
    }
}
