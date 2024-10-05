use std::f64::consts::PI;

use ast::{Node, Number};
use common::Language;
use evaluator::{
    evaluate_node, EndpointBuilder, EvaluateNodeError, HostEndpoint,
};

pub fn load_cos_endpoint(builder: EndpointBuilder) -> HostEndpoint {
    builder
        .description(Language::English, "tbd")
        .description(Language::German, "Berechnet den Cosinus von x.")
        .function(vec!["x"])
        .executor(|arguments, context| {
            if arguments.len() != 1 {
                return Err(EvaluateNodeError::RuntimeError(
                    "invalid number of arguments".into(),
                ));
            }
            let argument = arguments.get(0).unwrap();
            let evaluated_argument = evaluate_node(&argument, context)?;

            match evaluated_argument {
                Node::Number(Number { value }) => {
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
                _ => Err(EvaluateNodeError::RuntimeError(
                    "invalid argument type".into(),
                )),
            }
        })
}
