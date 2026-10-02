use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::reduced_row_echelon_form;
use node::{DowncastNodeVec, Number, Tensor};
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
            .downcast::<Number>()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .iter()
            .map(|number| number.value)
            .collect::<Vec<_>>();

        let (rows, columns) = match n.shape[..] {
            [r, c] => (r, c),
            _ => Err(Error::unreachable())?,
        };

        reduced_row_echelon_form(rows, columns, &mut matrix);

        Ok(Tensor::new_with_shape(
            vec![rows, columns],
            matrix.into_iter().map(Number::new_node).collect(),
        ))
    }
}
