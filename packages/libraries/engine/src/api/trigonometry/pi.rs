use std::f64::consts::PI;

use crate::core::{EndpointBuilder, HostEndpoint, Language, Number};

pub fn load_pi_endpoint(builder: EndpointBuilder) -> HostEndpoint {
    builder
        .description(Language::English, "Die Kreiszahl π ist allgemein definiert als das Verhältnis des Umfangs eines Kreises zu seinem Durchmesser.")
        .description(Language::German, "Die Kreiszahl π ist allgemein definiert als das Verhältnis des Umfangs eines Kreises zu seinem Durchmesser.")
        .constant(|_context| Number::new(PI).into())
}
