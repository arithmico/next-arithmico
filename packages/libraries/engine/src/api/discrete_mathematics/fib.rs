use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::fibonacci;
use node::Number;
use validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("fib")]
#[description(Language::German, "Berechnet die n-te Fibonacci Zahl.")]
#[description(Language::English, "Calculates the n-th fibonacci number")]
pub struct FibArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    n: &'a Number,
}

pub struct FibEndpoint;

impl FunctionEndpoint for FibEndpoint {
    type Output = Number;

    type Arguments<'a> = FibArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        FibArgs { n }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        n.validate_greater_than_or_equal(0.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let n = n.value as u64;
        let output = fibonacci(n);

        Ok(Number::new(output as f64))
    }
}
