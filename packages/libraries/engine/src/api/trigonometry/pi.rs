use std::f64::consts::PI;

use node::{Number, NodeType};

use crate::core::{EndpointBuilder, HostEndpoint, Language};

#[cfg(feature = "api_endpoint_trigonometry_pi")]
pub fn load_pi_endpoint(builder: EndpointBuilder) -> HostEndpoint {
    builder
        .name("pi")
        .description(Language::English, "The constant π is generally defined as the ratio of the circumference of a circle to its diameter.")
        .description(Language::German, "Die Kreiszahl π ist allgemein definiert als das Verhältnis des Umfangs eines Kreises zu seinem Durchmesser.")
        .node_type(NodeType::Number)
        .constant(|_context| Number::new(PI).into())
}
