use crate::{
    api::validations::NumberValidation,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
    Context,
};
use engine_derive::FunctionArguments;
use math_utils::calculate_binomial_pmf;
use node::Number;

#[derive(FunctionArguments)]
#[name("binom")]
#[description(
    Language::German,
    "Berechnet die Wahrscheinlichkeitsfunktion der Binomialverteilung."
)]
#[description(
    Language::English,
    "Computes the probability mass function of the binomial distribution."
)]
pub struct BinomArgs<'a> {
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

pub struct BinomEndpoint;

impl FunctionEndpoint for BinomEndpoint {
    type Output = Number;
    type Arguments<'a> = BinomArgs<'a>;

    fn executor<'a>(
        BinomArgs { n, p, k }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        p.validate_closed_interval(0.0, 1.0)?;

        n.validate_non_negative()?.validate_integer()?;
        k.validate_non_negative()?.validate_integer()?;

        calculate_binomial_pmf(n.value as usize, p.value, k.value as usize)
            .map_err(|_| EvaluateNodeError::runtime_error("binom"))
            .map(|result| Number::new(result))
    }
}
