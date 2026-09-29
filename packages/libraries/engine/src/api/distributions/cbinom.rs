use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::calculate_binomial_cdf;
use node::Number;
use validator::NumberValidator;

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

        calculate_binomial_cdf(n.value as usize, p.value, k.value as usize)
            .map_err(|_| Error::runtime_error("cbinom"))
            .map(Number::new)
    }
}
