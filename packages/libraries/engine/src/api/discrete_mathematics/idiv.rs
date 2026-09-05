use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use node::Number;
use validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("idiv")]
#[description(
    Language::German,
    "Berechnet den ganzzahligen Quotienten der Division von n durch m (Ganzzahldivision), wobei der Rest verworfen wird."
)]
#[description(
    Language::English,
    "Calculates the integer quotient of the division of n by m (integer division), discarding the remainder."
)]
pub struct IDivArgs<'a> {
    #[description(Language::German, "Divisor")]
    #[description(Language::English, "divisor")]
    n: &'a Number,

    #[description(Language::German, "Dividend")]
    #[description(Language::English, "dividend")]
    m: &'a Number,
}

pub struct IDivEndpoint;

impl FunctionEndpoint for IDivEndpoint {
    type Output = Number;
    type Arguments<'a> = IDivArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        IDivArgs { n, m }: Self::Arguments<'a>,
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

        Ok(Number::new(n_value.div_euclid(m_value)))
    }
}
