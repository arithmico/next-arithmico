use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::greatest_common_divisor;
use node::Number;
use validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("gcd")]
#[description(
    Language::German,
    "Berechnet den größten gemeinsamen Teiler (ggT)."
)]
#[description(
    Language::English,
    "Calculates the greatest common divisor (gcd)."
)]
pub struct GCDArgs<'a> {
    #[description(Language::German, "Erster Wert")]
    #[description(Language::English, "first value")]
    a: &'a Number,

    #[description(Language::German, "Zweiter Wert")]
    #[description(Language::English, "second value")]
    b: &'a Number,
}

pub struct GCDEndpoint;

impl FunctionEndpoint for GCDEndpoint {
    type Output = Number;
    type Arguments<'a> = GCDArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        GCDArgs { a, b }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        let a_value = a
            .validate_greater_than_or_equal(1.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .value as u64;
        let b_value = b
            .validate_greater_than_or_equal(1.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .value as u64;

        Ok(Number::new(greatest_common_divisor(a_value, b_value) as f64))
    }
}
