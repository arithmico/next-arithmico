use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use node::Number;
use node_validator::NumberValidator;

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
    fn executor<'a>(
        AsinArgs { x }: Self::Arguments<'a>,
        context: Options,
    ) -> Result<Self::Output, Error> {
        x.validate_inside_closed_interval(-1.0, 1.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let result = context.angle_unit.from_radians(x.value.asin());

        Ok(Number::new(result))
    }
}
