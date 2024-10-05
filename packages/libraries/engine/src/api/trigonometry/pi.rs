use std::f64::consts::PI;

use ast::Number;
use common::Language;
use evaluator::{EndpointBuilder, HostEndpoint};

pub fn load_pi_endpoint(builder: EndpointBuilder) -> HostEndpoint {
    builder
                .description(Language::English, "tbd")
                .description(Language::German, "Die Kreiszahl π ist allgemein definiert als das Verhältnis des Umfangs eines Kreises zu seinem Durchmesser.")
                .constant(|_context| Number::new(PI).into())
}
