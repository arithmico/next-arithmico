use crate::{
    api::validations::NumberValidation,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
    Context,
};
use engine_derive::FunctionArguments;
use math_utils::calculate_binomial_cdf;
use node::Number;

#[derive(FunctionArguments)]
#[name("cbinom")]
#[description(
    Language::German,
    "Berechnet die kumulierte Wahrscheinlichkeitsfunktion der Binomialverteilung."
)]
#[description(
    Language::English,
    "Computes the cumulative probability density function of the binomial distribution."
)]
pub struct CBinomArgs<'a> {
    #[description(Language::German, "Anzahl der Versuche")]
    #[description(Language::English, "number of trials")]
    n: &'a Number,

    #[description(Language::German, "Erfolgswahrscheinlichkeit")]
    #[description(Language::English, "success probability")]
    p: &'a Number,

    #[description(Language::German, "Anzahl der Erfolge")]
    #[description(Language::English, "number of successes")]
    k: &'a Number,
}

pub struct CBinomEndpoint;

impl FunctionEndpoint for CBinomEndpoint {
    type Output = Number;
    type Arguments<'a> = CBinomArgs<'a>;

    fn executor<'a>(
        CBinomArgs { n, p, k }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        p.validate_closed_interval(0.0, 1.0)?;

        n.validate_non_negative()?.validate_integer()?;
        k.validate_non_negative()?.validate_integer()?;

        calculate_binomial_cdf(n.value as usize, p.value, k.value as usize)
            .map_err(|_| EvaluateNodeError::runtime_error("cbinom"))
            .map(|result| Number::new(result))
    }
}
