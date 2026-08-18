use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use math_utils::calculate_unbiased_variance;
use node::Number;
use trace::{CombineHulls, TracableMut};

#[derive(FunctionArguments)]
#[name("unbiased:var")]
#[description(
    Language::German,
    "Berechnet die normierte Stichprobenvarianz (geteilt durch n - 1)."
)]
#[description(
    Language::English,
    "Calculates the unbiased sample variance (divided by n - 1)."
)]
pub struct UnbiasedVarArgs<'a> {
    #[repeatable(min = 2)]
    #[description(Language::German, "Werte")]
    #[description(Language::English, "values")]
    x: Vec<&'a Number>,
}

pub struct UnbiasedVarEndpoint;

impl FunctionEndpoint for UnbiasedVarEndpoint {
    type Output = Number;
    type Arguments<'a> = UnbiasedVarArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        UnbiasedVarArgs { x }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        let values = x.iter().map(|number| number.value).collect::<Vec<_>>();
        let value = calculate_unbiased_variance(&values).ok_or_else(|| {
            Error::unreachable().with_optional_span(x.combine_hulls())
        })?;

        Ok(Number::new(value))
    }
}
