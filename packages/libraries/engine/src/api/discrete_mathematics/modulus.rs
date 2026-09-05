use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use node::Number;
use validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("mod")]
#[description(
    Language::German,
    "Berechnet den mathematischen Rest der Division von n durch m (modulus), wobei das Vorzeichen dem des Divisors m entspricht."
)]
#[description(
    Language::English,
    "Calculates the mathematical remainder of the division of n by m (modulus), where the sign matches that of the divisor m."
)]
pub struct ModArgs<'a> {
    #[description(Language::German, "Divisor")]
    #[description(Language::English, "divisor")]
    n: &'a Number,

    #[description(Language::German, "Dividend")]
    #[description(Language::English, "dividend")]
    m: &'a Number,
}

pub struct ModEndpoint;

impl FunctionEndpoint for ModEndpoint {
    type Output = Number;
    type Arguments<'a> = ModArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        ModArgs { n, m }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        let n_value = n
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .value;
        let m_value = m
            .validate_not_equal(0.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .value;

        Ok(Number::new(n_value.rem_euclid(m_value)))
    }
}
