use std::collections::HashSet;

use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::calculate_correlation_coefficient;
use node::{GetNodeType, NodeType, Number, Tensor};
use trace::{Tracable, TracableMut};
use validator::TensorValidator;

#[derive(FunctionArguments)]
#[name("corr")]
#[description(
    Language::German,
    "Berechnet den Korrelationskoeffizienten (Pearson) zweier gleichgroßer Vektoren."
)]
#[description(
    Language::English,
    "Calculates the Pearson correlation coefficient of two vectors of the same size."
)]
pub struct CorrArgs<'a> {
    #[description(Language::German, "Erster Wertevektor")]
    #[description(Language::English, "first value vector")]
    xs: &'a Tensor,

    #[description(Language::German, "Zweiter Wertevektor")]
    #[description(Language::English, "second value vector")]
    ys: &'a Tensor,
}

pub struct CorrEndpoint;

impl FunctionEndpoint for CorrEndpoint {
    type Output = Number;
    type Arguments<'a> = CorrArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        CorrArgs { xs, ys }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        xs.validate_minimum_vector_length(2)
            .map_to_error_kind(ErrorKind::IncompatibleVectorDimensions)?;
        ys.validate_minimum_vector_length(2)
            .map_to_error_kind(ErrorKind::IncompatibleVectorDimensions)?;

        xs.validate_equal_shapes(ys)
            .map_to_error_kind(ErrorKind::IncompatibleVectorDimensions)?;

        let x_values = xs
            .elements
            .iter()
            .map(|node| {
                <&Number>::try_from(node)
                    .map(|number| number.value)
                    .map_err(|_| {
                        Error::invalid_parameter_type(
                            "xs",
                            HashSet::from([NodeType::Number]),
                            xs.node_type(),
                        )
                    })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let y_values = ys
            .elements
            .iter()
            .map(|node| {
                <&Number>::try_from(node)
                    .map(|number| number.value)
                    .map_err(|_| {
                        Error::invalid_parameter_type(
                            "ys",
                            HashSet::from([NodeType::Number]),
                            ys.node_type(),
                        )
                    })
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let result = calculate_correlation_coefficient(&x_values, &y_values)
            .ok_or_else(|| {
                Error::unreachable()
                    .with_optional_span(xs.hull())
                    .with_optional_span(ys.hull())
            })?;

        Ok(Number::new(result))
    }
}
