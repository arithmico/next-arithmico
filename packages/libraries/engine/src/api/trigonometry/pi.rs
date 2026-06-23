use std::f64::consts::PI;

use crate::core::{EndpointBuilder, HostEndpoint, Language};

#[cfg(feature = "api_endpoint_trigonometry_pi")]
pub fn load_pi_endpoint(builder: EndpointBuilder) -> HostEndpoint {
    use node::Number;

    builder
        .name("pi")
        .description(Language::English, "The constant π is generally defined as the ratio of the circumference of a circle to its diameter.")
        .description(Language::German, "Die Kreiszahl π ist allgemein definiert als das Verhältnis des Umfangs eines Kreises zu seinem Durchmesser.")
        .constant(|_context| Number::new_node(PI).into())
}
