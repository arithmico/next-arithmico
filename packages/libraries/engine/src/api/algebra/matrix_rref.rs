use std::collections::HashSet;

use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::reduced_row_echelon_form;
use node::{GetNodeType, NodeType, Number, Tensor};
use validator::TensorValidator;

#[derive(FunctionArguments)]
#[name("matrix:rref")]
#[description(
    Language::German,
    "Gibt die reduzierte Zeilenstufenform einer n x m Matrix zurück."
)]
#[description(
    Language::English,
    "Returns the reduced row echelon form of an n x m matrix."
)]
pub struct MatrixRrefArgs<'a> {
    #[description(Language::German, "Matrix")]
    #[description(Language::English, "matrix")]
    n: &'a Tensor,
}

pub struct MatrixRrefEndpoint;

impl FunctionEndpoint for MatrixRrefEndpoint {
    type Output = Tensor;
    type Arguments<'a> = MatrixRrefArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        MatrixRrefArgs { n }: Self::Arguments<'a>,
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

        reduced_row_echelon_form(rows, columns, &mut matrix);

        Ok(Tensor::new_with_shape(
            vec![rows, columns],
            matrix.into_iter().map(|v| Number::new_node(v)).collect(),
        ))
    }
}
