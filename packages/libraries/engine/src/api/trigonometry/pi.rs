use std::f64::consts::PI;

use common::Language;
use engine_derive::ConstantMetadata;
use evaluator::{ConstantEndpoint, Options};
use node::Number;

#[derive(ConstantMetadata)]
#[name("pi")]
#[description(
    Language::German,
    "Die Kreiszahl π ist allgemein definiert als das Verhältnis des Umfangs eines Kreises zu seinem Durchmesser."
)]
#[description(
    Language::English,
    "The constant π is generally defined as the ratio of the circumference of a circle to its diameter."
)]
pub struct PiEndpoint;

impl ConstantEndpoint for PiEndpoint {
    type Output = Number;

    fn executor(_context: Options) -> Self::Output {
        Number::new(PI)
    }
}
