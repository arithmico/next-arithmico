use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use node::Number;
use node_validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("root")]
#[description(Language::German, "Berechnet die n-te Wurzel von x.")]
#[description(Language::English, "Calculates the n-th root of x.")]
pub struct RootArgs<'a> {
    #[description(Language::German, "Radikant")]
    #[description(Language::English, "radicant")]
    x: &'a Number,

    #[description(Language::German, "Wurzelexponent")]
    #[description(Language::English, "root exponent")]
    n: &'a Number,
}

pub struct RootEndpoint;

impl FunctionEndpoint for RootEndpoint {
    type Output = Number;

    type Arguments<'a> = RootArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        RootArgs { x, n }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        n.validate_greater_than_or_equal(0.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let value = if n.value.rem_euclid(2.0) == 0.0 {
            x.validate_positive()
                .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

            x.value.powf(1.0 / n.value.abs())
        } else {
            x.value.signum() * x.value.abs().powf(1.0 / n.value.abs())
        };

        Ok(Number::new(value))
    }
}
