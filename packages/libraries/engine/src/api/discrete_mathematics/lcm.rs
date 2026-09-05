use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::least_common_multiple;
use node::Number;
use validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("lcm")]
#[description(
    Language::German,
    "Berechnet das kleinste gemeinsame Vielfache (kgV)."
)]
#[description(Language::English, "Calculates the least common multiple (lcm).")]
pub struct LCMArgs<'a> {
    #[description(Language::German, "Erster Wert")]
    #[description(Language::English, "first value")]
    a: &'a Number,

    #[description(Language::German, "Zweiter Wert")]
    #[description(Language::English, "second value")]
    b: &'a Number,
}

pub struct LCMEndpoint;

impl FunctionEndpoint for LCMEndpoint {
    type Output = Number;
    type Arguments<'a> = LCMArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        LCMArgs { a, b }: Self::Arguments<'a>,
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

        Ok(Number::new(least_common_multiple(a_value, b_value) as f64))
    }
}
