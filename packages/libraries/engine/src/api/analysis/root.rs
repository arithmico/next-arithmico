use engine_derive::FunctionArguments;
use node::Number;
use node_validator::NumberValidator;

use crate::{
    Context,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
};

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
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        n.validate_greater_than_or_equal(0.0)?.validate_integer()?;

        let value = if n.value.rem_euclid(2.0) == 0.0 {
            x.validate_positive()?;
            x.value.powf(1.0 / n.value.abs())
        } else {
            x.value.signum() * x.value.abs().powf(1.0 / n.value.abs())
        };

        Ok(Number::new(value))
    }
}
