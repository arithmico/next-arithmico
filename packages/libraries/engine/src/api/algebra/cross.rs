use engine_derive::FunctionArguments;
use evaluator::{Error, ErrorKind, MapToEvaluatorError};
use node::{DowncastNodeVec, Number, Tensor};
use node_validator::TensorValidator;

use crate::{
    Context,
    core::{FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("cross")]
#[description(
    Language::German,
    "Berechnet das Kreuzprodukt der Vektoren a und b."
)]
#[description(Language::English, "Calculate the vector product of a and b.")]
pub struct CrossArgs<'a> {
    #[description(Language::German, "Vektor")]
    #[description(Language::English, "vector")]
    a: &'a Tensor,

    #[description(Language::German, "Vektor")]
    #[description(Language::English, "vector")]
    b: &'a Tensor,
}

pub struct CrossEndpoint;

impl FunctionEndpoint for CrossEndpoint {
    type Output = Tensor;

    type Arguments<'a> = CrossArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        CrossArgs { a, b }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, Error> {
        a.validate_rank(1)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_vector_dimension(3)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;
        b.validate_rank(1)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_vector_dimension(3)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;
        let a = a
            .elements
            .downcast::<Number>()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;
        let b = b
            .elements
            .downcast::<Number>()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let output = match (&a[..], &b[..]) {
            ([a1, a2, a3], [b1, b2, b3]) => vec![
                Number::new_node(a2.value * b3.value - a3.value * b2.value),
                Number::new_node(a3.value * b1.value - a1.value * b3.value),
                Number::new_node(a1.value * b2.value - a2.value * b1.value),
            ],
            _ => unreachable!("already validated before"),
        };

        Ok(Tensor::new(output))
    }
}
