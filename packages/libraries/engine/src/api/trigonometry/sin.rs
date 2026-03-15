use std::f64::consts::PI;

use engine_derive::FromArgumentsBinding;

use crate::core::{
    EndpointBuilder, FunctionSignature, HostEndpoint, Language, NodeType,
    Number,
};

#[derive(FromArgumentsBinding)]
pub struct SinArgs {
    value: Number,
}

pub fn load_sin_endpoint(builder: EndpointBuilder) -> HostEndpoint {
    let signature = FunctionSignature::new()
        .argument("x", |argument| {
            argument
                .evaluate()
                .description(Language::English, "angle")
                .description(Language::German, "Winkel")
                .node_type(NodeType::Number)
        })
        .add_return_type(NodeType::Number);

    builder
        .description(Language::English, "Calculate the sine of x.")
        .description(Language::German, "Berechnet den Sinus von x.")
        .function(signature)
        .executor(|SinArgs { value }, _context| {
            let value = value.value;

            if value.rem_euclid(PI).abs() < value * f64::EPSILON {
                return Ok(Number::new(0.0));
            }

            Ok(Number::new(value.sin()))
        })
}
