use std::f64::consts::PI;

use ast::{
    EndpointBuilder, EvaluateNodeError, FunctionSignature, HostEndpoint,
    Language, Node, NodeType, Number,
};

pub fn load_cos_endpoint(builder: EndpointBuilder) -> HostEndpoint {
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
        .description(Language::English, "Calculate the consine of x.")
        .description(Language::German, "Berechnet den Cosinus von x.")
        .function(signature)
        .executor(|arguments, _context| {
            let argument = arguments.get_parameter_value("x")?;

            match argument {
                Node::Number(Number { value, .. }) => {
                    let modulus_pi = value.rem_euclid(PI).abs();
                    if modulus_pi < value * f64::EPSILON {
                        return Ok(Number::new(1.));
                    } else if modulus_pi.rem_euclid(PI / 2.).abs()
                        < value * f64::EPSILON
                    {
                        return Ok(Number::new(0.));
                    }

                    Ok(Number::new(value.cos()))
                }
                _ => Err(EvaluateNodeError::runtime_error(
                    "invalid argument type",
                )),
            }
        })
}
