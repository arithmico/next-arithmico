use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use math_utils::calculate_unbiased_standard_deviation;
use node::Number;
use trace::{CombineHulls, TracableMut};

#[derive(FunctionArguments)]
#[name("unbiased:sd")]
#[description(
    Language::German,
    "Berechnet die Standardabweichung basierend auf der normierten Stichprobenvarianz."
)]
#[description(
    Language::English,
    "Calculates the standard deviation based on unbiased sample variance."
)]
pub struct UnbiasedSdArgs<'a> {
    #[repeatable(min = 2)]
    #[description(Language::German, "Werte")]
    #[description(Language::English, "values")]
    x: Vec<&'a Number>,
}

pub struct UnbiasedSdEndpoint;

impl FunctionEndpoint for UnbiasedSdEndpoint {
    type Output = Number;
    type Arguments<'a> = UnbiasedSdArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        UnbiasedSdArgs { x }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        let values = x.iter().map(|number| number.value).collect::<Vec<_>>();
        let value = calculate_unbiased_standard_deviation(&values).ok_or_else(
            || Error::unreachable().with_optional_span(x.combine_hulls()),
        )?;

        Ok(Number::new(value))
    }
}
