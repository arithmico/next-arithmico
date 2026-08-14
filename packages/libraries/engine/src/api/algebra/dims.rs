use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use node::{Number, Tensor};

#[derive(FunctionArguments)]
#[name("tensor:dims")]
#[description(Language::German, "Berechnet die Dimensionen eines Tensors.")]
#[description(Language::English, "Calculates the dimensions of a tensor.")]
pub struct DimskArgs<'a> {
    #[description(Language::German, "Tensor")]
    #[description(Language::English, "tensor")]
    x: &'a Tensor,
}

pub struct DimsEndpoint;

impl FunctionEndpoint for DimsEndpoint {
    type Output = Tensor;

    type Arguments<'a> = DimskArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        DimskArgs { x }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        Ok(Tensor::new(
            x.shape
                .iter()
                .map(|v| Number::new_node(*v as f64))
                .collect::<Vec<_>>(),
        ))
    }
}
