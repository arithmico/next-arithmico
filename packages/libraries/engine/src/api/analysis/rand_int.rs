use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::random_integer;
use node::Number;
use validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("rand:int")]
#[description(
    Language::German,
    "Gibt eine zufällige Ganzzahl zwischen min und max zurück."
)]
#[description(
    Language::English,
    "Returns a random integer between min and max."
)]
pub struct RandIntArgs<'a> {
    #[description(Language::German, "Erster Wert")]
    #[description(Language::English, "first value")]
    min: &'a Number,

    #[description(Language::German, "Zweiter Wert")]
    #[description(Language::English, "second value")]
    max: &'a Number,
}

pub struct RandIntEndpoint;

impl FunctionEndpoint for RandIntEndpoint {
    type Output = Number;
    type Arguments<'a> = RandIntArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        RandIntArgs { min, max }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        let min_value = min
            .validate_less_than_or_equal(max.value)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .value as i64;
        let max_value = max
            .validate_greater_than(min.value)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .value as i64;

        Ok(Number::new(random_integer(min_value, max_value) as f64))
    }
}
