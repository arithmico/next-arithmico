use engine_derive::FunctionArguments;
use evaluator::{Error, ErrorKind, MapToEvaluatorError};
use node::Number;
use node_validator::NumberValidator;

use crate::{
    Context,
    core::{FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("acos")]
#[description(Language::German, "Berechnet den Arkuscosinus von x.")]
#[description(Language::English, "Calculates the arc cosine of x.")]
pub struct AcosArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,
}

pub struct AcosEndpoint;

impl FunctionEndpoint for AcosEndpoint {
    type Output = Number;

    type Arguments<'a> = AcosArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        AcosArgs { x }: Self::Arguments<'a>,
        context: &Context,
    ) -> Result<Self::Output, Error> {
        x.validate_inside_closed_interval(-1.0, 1.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let result = context.angle_unit.from_radians(x.value.acos());

        Ok(Number::new(result))
    }
}
