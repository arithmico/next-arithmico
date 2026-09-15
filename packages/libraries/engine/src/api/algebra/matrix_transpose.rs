use std::collections::HashSet;

use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::transpose;
use node::{GetNodeType, NodeType, Number, Tensor};
use validator::TensorValidator;

#[derive(FunctionArguments)]
#[name("matrix:transpose")]
#[description(Language::German, "Transponiert eine n x m Matrix.")]
#[description(Language::English, "Transposes an n x m matrix.")]
pub struct MatrixTransposeArgs<'a> {
    #[description(Language::German, "Matrix")]
    #[description(Language::English, "matrix")]
    n: &'a Tensor,
}

pub struct MatrixTransposeEndpoint;

impl FunctionEndpoint for MatrixTransposeEndpoint {
    type Output = Tensor;
    type Arguments<'a> = MatrixTransposeArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        MatrixTransposeArgs { n }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        n.validate_matrix()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let mut matrix = n
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

        let (rows, columns) = match n.shape[..] {
            [r, c] => (r, c),
            _ => Err(Error::unreachable())?,
        };

        transpose(rows, columns, &mut matrix);

        Ok(Tensor::new_with_shape(
            vec![columns, rows],
            matrix.into_iter().map(|v| Number::new_node(v)).collect(),
        ))
    }
}
