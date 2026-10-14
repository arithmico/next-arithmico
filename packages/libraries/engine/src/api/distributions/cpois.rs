use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::calculate_poisson_cdf;
use node::Number;
use validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("cpois")]
#[description(
    Language::German,
    "Berechnet die kumulierte Wahrscheinlichkeitsfunktion der Poissonverteilung."
)]
#[description(
    Language::English,
    "Computes the cumulative distribution function of the Poisson distribution."
)]
pub struct CPoisArgs<'a> {
    #[description(Language::German, "Anzahl der Ereignisse")]
    #[description(Language::English, "number of events")]
    k: &'a Number,

    #[description(Language::German, "Erwartete Anzahl der Ereignisse")]
    #[description(Language::English, "expected number of events")]
    lambda: &'a Number,
}

pub struct CPoisEndpoint;

impl FunctionEndpoint for CPoisEndpoint {
    type Output = Number;
    type Arguments<'a> = CPoisArgs<'a>;

    fn executor<'a>(
        CPoisArgs { k, lambda }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        k.validate_positive()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;
        lambda
            .validate_greater_than(0.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        calculate_poisson_cdf(k.value as usize, lambda.value)
            .map_err(|_| Error::runtime_error("cpois"))
            .map(Number::new)
    }
}
