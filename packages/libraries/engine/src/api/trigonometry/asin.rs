use engine_derive::FunctionArguments;
use node::Number;

use crate::{
    Context, api::validations::NumberValidation, core::{EvaluateNodeError, FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("asin")]
#[description(Language::German, "Berechnet den Arkussinus von x.")]
#[description(Language::English, "Calculates the arcsine of x.")]
pub struct AsinArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct AsinEndpoint;

impl FunctionEndpoint for AsinEndpoint {
    type Output = Number;

    type Arguments<'a> = AsinArgs<'a>;

    // TODO: unit tests
    // TODO: handle special values
    fn executor<'a>(
        AsinArgs { x }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        x.validate_closed_interval(-1.0, 1.0)?;

        let result = x.value.asin();

        Ok(Number::new(result))
    }
}
