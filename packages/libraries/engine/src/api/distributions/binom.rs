use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::calculate_binomial_pmf;
use node::Number;
use validator::NumberValidator;

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
        _context: Options,
    ) -> Result<Self::Output, Error> {
        p.validate_inside_closed_interval(0.0, 1.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;
        n.validate_positive()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;
        k.validate_positive()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        calculate_binomial_pmf(n.value as usize, p.value, k.value as usize)
            .map_err(|_| Error::runtime_error("binom"))
            .map(Number::new)
    }
}
