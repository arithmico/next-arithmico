use engine_derive::FunctionArguments;
use evaluator::{Error, ErrorKind, MapToEvaluatorError};
use node::{Node, Number, Tensor};
use node_validator::NumberValidator;

use crate::{
    Context,
    core::{FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("matrix:id")]
#[description(Language::German, "Erzeugt eine n x n Einheitsmatrix.")]
#[description(Language::English, "Creates an n x n identity matrix.")]
pub struct MatrixIdArgs<'a> {
    #[description(Language::German, "Dimension")]
    #[description(Language::English, "dimension")]
    n: &'a Number,
}

pub struct MatrixIdEndpoint;

impl FunctionEndpoint for MatrixIdEndpoint {
    type Output = Tensor;

    type Arguments<'a> = MatrixIdArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        MatrixIdArgs { n }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, Error> {
        n.validate_greater_than(0.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let n = n.value as usize;
        let mut elements = Vec::<Node>::with_capacity(n);
        for i in 0..n {
            let mut row = Vec::<Node>::with_capacity(n);
            for j in 0..n {
                if i == j {
                    row.push(Number::new_node(1.0));
                } else {
                    row.push(Number::new_node(0.0));
                }
            }
            elements.push(Tensor::new_node(row));
        }

        Ok(Tensor::new(elements))
    }
}
