use std::f64::consts::PI;

use ast::{FunctionSignature, Node, NodeType, Number};
use common::{EndpointBuilder, EvaluateNodeError, HostEndpoint, Language};

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
        .executor(|arguments, _context| {
            let argument = arguments.get_parameter_value("x")?;

            match argument {
                Node::Number(Number { value, .. }) => {
                    if value.rem_euclid(PI).abs() < value * f64::EPSILON {
                        return Ok(Number::new(0.0));
                    }
                    Ok(Number::new(value.sin()))
                }
                node => Err(EvaluateNodeError::runtime_error(
                    "invalid argument type",
                )
                .with_tracable(node)),
            }
        })
}
