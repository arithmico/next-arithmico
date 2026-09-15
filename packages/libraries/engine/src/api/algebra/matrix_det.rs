use std::collections::HashSet;

use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::matrix_determinant;
use node::{GetNodeType, NodeType, Number, Tensor};
use validator::TensorValidator;

#[derive(FunctionArguments)]
#[name("matrix:det")]
#[description(
    Language::German,
    "Berechnet die Determinante einer n x n Matrix."
)]
#[description(
    Language::English,
    "Calculates the determinant of an n x n matrix."
)]
pub struct MatrixDetArgs<'a> {
    #[description(Language::German, "Matrix")]
    #[description(Language::English, "matrix")]
    n: &'a Tensor,
}

pub struct MatrixDetEndpoint;

impl FunctionEndpoint for MatrixDetEndpoint {
    type Output = Number;
    type Arguments<'a> = MatrixDetArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        MatrixDetArgs { n }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        n.validate_square_matrix()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let matrix = n
            .elements
            .iter()
            .map(|node| {
                <&Number>::try_from(node)
                    .map(|number| number.value)
                    .map_err(|_| {
                        Error::invalid_parameter_type(
                            "n",
                            HashSet::from([NodeType::Number]),
                            node.node_type(),
                        )
                    })
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let dim = n.shape[0];

        Ok(Number::new(matrix_determinant(dim, &matrix)))
    }
}
