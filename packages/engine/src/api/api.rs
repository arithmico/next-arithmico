use std::f64::consts::PI;

use crate::{context::HostApi, evaluate::NodeEvaluationError, node::Node};

pub fn load_host_api() -> HostApi {
    HostApi::builder()
        .add_constant_endpoint("pi".into(), |_context| Node::Number {
            value: PI,
        })
        .add_function_endpoint("sin", |arguments, context| {
            if arguments.len() != 1 {
                return Err(NodeEvaluationError::RuntimeError(
                    "invalid number of arguments".into(),
                ));
            }
            let argument = arguments.get(0).unwrap();
            let evaluated_argument = argument.evaluate(context)?;
            match evaluated_argument {
                Node::Number { value } => {
                    if value.rem_euclid(PI).abs() < value * f64::EPSILON {
                        return Ok(Node::Number { value: 0.0 });
                    }
                    Ok(Node::Number { value: value.sin() })
                }
                _ => Err(NodeEvaluationError::RuntimeError(
                    "invalid argument type".into(),
                )),
            }
        })
        .build()
}
