use std::f64::consts::PI;

#[cfg(feature = "api_endpoint_trigonometry_pi")]
use engine_derive::ConstantMetadata;
use node::Number;

use crate::{
    Context,
    core::{ConstantEndpoint, Language},
};

#[cfg(feature = "api_endpoint_trigonometry_pi")]
#[derive(ConstantMetadata)]
#[name("pi")]
#[description(Language::German, "Die Kreiszahl π ist allgemein definiert als das Verhältnis des Umfangs eines Kreises zu seinem Durchmesser.")]
#[description(Language::English, "The constant π is generally defined as the ratio of the circumference of a circle to its diameter.")]
pub struct PiEndpoint;

impl ConstantEndpoint for PiEndpoint {
    type Output = Number;

    fn executor(_context: &Context) -> Self::Output {
        Number::new(PI)
    }
}
