use engine_derive::ConstantMetadata;
use node::Number;
use translate::Language;

use crate::{Context, core::ConstantEndpoint};

#[derive(ConstantMetadata)]
#[name("e")]
#[description(
    Language::German,
    "Repraesentiert die eulersche Zahl e, die Basis des natuerlichen Logarithmus."
)]
#[description(
    Language::English,
    "Represents Euler's number e, the base of the natural logarithm"
)]
pub struct EEndpoint;

impl ConstantEndpoint for EEndpoint {
    type Output = Number;

    fn executor(_context: &Context) -> Self::Output {
        Number::new(std::f64::consts::E)
    }
}
