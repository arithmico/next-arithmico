use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use math_utils::calculate_biased_variance;
use node::Number;
use trace::{CombineHulls, TracableMut};

#[derive(FunctionArguments)]
#[name("var")]
#[description(
    Language::German,
    "Berechnet die Stichprobenvarianz (geteilt durch n)."
)]
#[description(
    Language::English,
    "Calculates the biased sample variance (divided by n)."
)]
pub struct VarArgs<'a> {
    #[repeatable(min = 2)]
    #[description(Language::German, "Werte")]
    #[description(Language::English, "values")]
    x: Vec<&'a Number>,
}

pub struct VarEndpoint;

impl FunctionEndpoint for VarEndpoint {
    type Output = Number;
    type Arguments<'a> = VarArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        VarArgs { x }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        let values = x.iter().map(|number| number.value).collect::<Vec<_>>();
        let value = calculate_biased_variance(&values).ok_or_else(|| {
            Error::unreachable().with_optional_span(x.combine_hulls())
        })?;

        Ok(Number::new(value))
    }
}
