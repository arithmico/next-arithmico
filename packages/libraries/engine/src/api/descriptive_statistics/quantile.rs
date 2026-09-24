use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::calculate_sample_quantile;
use node::{DowncastNodeVec, Number, Tensor};
use validator::{NumberValidator, TensorValidator};

#[derive(FunctionArguments)]
#[name("quantile")]
#[description(
    Language::German,
    "Berechnet das p-Quantil, d. h. p (zwischen 0 und 1) teilt die Menge auf in einen Teil p kleiner oder gleich und einen anderen Teil 1-p größer oder gleich dem Quantil ist."
)]
#[description(
    Language::English,
    "Calculates the p-quantile, i.e. p (between 0 and 1) divides the quantity into a part p less than or equal to and another part 1-p is greater than or equal to the quantile."
)]
pub struct QuantileArgs<'a> {
    #[description(Language::German, "Quantil")]
    #[description(Language::English, "quantile")]
    p: &'a Number,

    #[description(Language::German, "Werte")]
    #[description(Language::English, "values")]
    xs: &'a Tensor,
}

pub struct QuantileEndpoint;

impl FunctionEndpoint for QuantileEndpoint {
    type Output = Number;
    type Arguments<'a> = QuantileArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        QuantileArgs { p, xs }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        p.validate_inside_closed_interval(0.0, 1.0)
            .map_to_error_kind(evaluator::ErrorKind::InvalidParameterType)?;

        xs.validate_minimum_vector_length(2)
            .map_to_error_kind(ErrorKind::IncompatibleVectorDimensions)?;

        let mut x_values = xs
            .downcast::<Number>()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .iter()
            .map(|number| number.value)
            .collect::<Vec<_>>();

        let value = calculate_sample_quantile(p.value, &mut x_values);

        Ok(Number::new(value))
    }
}
