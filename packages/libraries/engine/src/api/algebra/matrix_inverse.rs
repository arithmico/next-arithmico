use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::inverse;
use node::{DowncastNodeVec, Number, Tensor};
use validator::TensorValidator;

#[derive(FunctionArguments)]
#[name("matrix:inverse")]
#[description(Language::German, "Invertiert eine n x n Matrix, falls möglich.")]
#[description(Language::English, "Inverts an n x n matrix if possible.")]
pub struct MatrixInverseArgs<'a> {
    #[description(Language::German, "Matrix")]
    #[description(Language::English, "matrix")]
    n: &'a Tensor,
}

pub struct MatrixInverseEndpoint;

impl FunctionEndpoint for MatrixInverseEndpoint {
    type Output = Tensor;
    type Arguments<'a> = MatrixInverseArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        MatrixInverseArgs { n }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        n.validate_square_matrix()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let mut matrix = n
            .downcast::<Number>()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .iter()
            .map(|number| number.value)
            .collect::<Vec<_>>();

        let n = match n.shape[..] {
            [r, c] if r == c => r,
            _ => Err(Error::unreachable())?,
        };

        inverse(n, &mut matrix);

        Ok(Tensor::new_with_shape(
            vec![n, n],
            matrix.into_iter().map(Number::new_node).collect(),
        ))
    }
}
