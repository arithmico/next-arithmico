use engine_derive::FunctionArguments;
use evaluator::{Error, ErrorKind, MapToEvaluatorError};
use node::Number;
use node_validator::NumberValidator;

use crate::{
    Context,
    core::{FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("atanh")]
#[description(
    Language::German,
    "Berechnet den Area Tangens hyperbolicus (die Umkehrfunktion des Tangens hyperbolicus) von x."
)]
#[description(
    Language::English,
    "Calculates the inverse hyperbolic tangent of x."
)]
pub struct AtanhArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct AtanhEndpoint;

impl FunctionEndpoint for AtanhEndpoint {
    type Output = Number;

    type Arguments<'a> = AtanhArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        AtanhArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, Error> {
        let value = x.value;

        x.validate_inside_open_interval(-1.0, 1.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let result = value.atanh();
        Ok(Number::new(result))
    }
}
