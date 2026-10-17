use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use node::Number;

#[derive(FunctionArguments)]
#[name("sum:squares")]
#[description(
    Language::German,
    "Gibt die Summe der quadrierten Elemente zurück."
)]
#[description(Language::English, "Returns the sum of the squared elements.")]
pub struct SumSquaresArgs<'a> {
    #[description(Language::German, "Werte")]
    #[description(Language::English, "values")]
    x: Vec<&'a Number>,
}

pub struct SumSquaresEndpoint;

impl FunctionEndpoint for SumSquaresEndpoint {
    type Output = Number;
    type Arguments<'a> = SumSquaresArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        SumSquaresArgs { x }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        let value = match &x[..] {
            [] => 0.0,
            [item] => item.value.powi(2),
            items => items.iter().map(|node| node.value.powi(2)).sum(),
        };

        Ok(Number::new(value))
    }
}
