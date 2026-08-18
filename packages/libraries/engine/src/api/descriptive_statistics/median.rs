use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use math_utils::calculate_sample_quantile;
use node::Number;

#[derive(FunctionArguments)]
#[name("median")]
#[description(
    Language::German,
    "Berechnet den Median der übergebenen Argumente."
)]
#[description(
    Language::English,
    "Calculates the median of the given arguments."
)]
pub struct MedianArgs<'a> {
    #[repeatable(min = 2)]
    #[description(Language::German, "Werte")]
    #[description(Language::English, "values")]
    x: Vec<&'a Number>,
}

pub struct MedianEndpoint;

impl FunctionEndpoint for MedianEndpoint {
    type Output = Number;
    type Arguments<'a> = MedianArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        MedianArgs { x }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        let mut values =
            x.iter().map(|number| number.value).collect::<Vec<_>>();
        let value = calculate_sample_quantile(0.5, &mut values);

        Ok(Number::new(value))
    }
}
