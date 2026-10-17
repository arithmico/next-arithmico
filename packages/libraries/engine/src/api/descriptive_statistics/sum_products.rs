use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use node::{DowncastNodeVec, Number, Tensor};
use validator::TensorValidator;

#[derive(FunctionArguments)]
#[name("sum:products")]
#[description(
    Language::German,
    "Gibt die Summe der Produkte der entsprechenden Elemente zurück."
)]
#[description(
    Language::English,
    "Returns the sum of the products of corresponding elements."
)]
pub struct SumProductsArgs<'a> {
    #[description(Language::German, "Erster Wertevektor")]
    #[description(Language::English, "first value vector")]
    xs: &'a Tensor,

    #[description(Language::German, "Zweiter Wertevektor")]
    #[description(Language::English, "second value vector")]
    ys: &'a Tensor,
}

pub struct SumProductsEndpoint;

impl FunctionEndpoint for SumProductsEndpoint {
    type Output = Number;
    type Arguments<'a> = SumProductsArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        SumProductsArgs { xs, ys }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        xs.validate_minimum_vector_length(1)
            .map_to_error_kind(ErrorKind::IncompatibleVectorDimensions)?;
        ys.validate_minimum_vector_length(1)
            .map_to_error_kind(ErrorKind::IncompatibleVectorDimensions)?;

        xs.validate_equal_shapes(ys)
            .map_to_error_kind(ErrorKind::IncompatibleVectorDimensions)?;

        let xs = xs
            .downcast::<Number>()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;
        let ys = ys
            .downcast::<Number>()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        Ok(Number::new(
            xs.into_iter().zip(ys).map(|(x, y)| x.value * y.value).sum(),
        ))
    }
}
