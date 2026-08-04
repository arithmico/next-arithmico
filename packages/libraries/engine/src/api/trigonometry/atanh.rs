use engine_derive::FunctionArguments;
use node::Number;

use crate::{
    Context,
    api::validations::NumberValidation,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
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
    ) -> Result<Self::Output, EvaluateNodeError> {
        let value = x.value;

        x.validate_open_interval(-1.0, 1.0)?;

        let result = value.atanh();
        Ok(Number::new(result))
    }
}
