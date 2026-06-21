use std::f64::consts::PI;

use engine_derive::FromArgumentMapping;
use node::{FunctionSignature, Node, NodeType, Number};

use crate::{
    core::{EndpointBuilder, EvaluateNodeError, HostEndpoint, Language},
    function_executor_wrapper, Context,
};

#[derive(FromArgumentMapping)]
struct CosArgs<'a> {
    x: &'a Number,
}

fn cos_executor(
    CosArgs { x }: CosArgs,
    _context: &Context,
) -> Result<Node, EvaluateNodeError> {
    let value = x.value;

    let modulus_pi = value.rem_euclid(PI).abs();
    if modulus_pi < value * f64::EPSILON {
        return Ok(Number::new(1.));
    } else if modulus_pi.rem_euclid(PI / 2.).abs() < value * f64::EPSILON {
        return Ok(Number::new(0.));
    }

    Ok(Number::new(value.cos()))
}

#[cfg(feature = "api_endpoint_trigonometry_cos")]
pub fn load_cos_endpoint(builder: EndpointBuilder) -> HostEndpoint {
    let signature = FunctionSignature::new()
        .argument("x", |argument| {
            use node::NodeType;

            argument
                .evaluate()
                .description(Language::English, "angle")
                .description(Language::German, "Winkel")
                .node_type(NodeType::Number)
        })
        .add_return_type(NodeType::Number);

    builder
        .name("cos")
        .description(Language::English, "Calculate the consine of x.")
        .description(Language::German, "Berechnet den Cosinus von x.")
        .function(signature)
        .executor(function_executor_wrapper!(CosArgs, cos_executor))
}
