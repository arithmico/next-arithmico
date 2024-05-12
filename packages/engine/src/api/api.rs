use std::{collections::HashMap, f64::consts::PI};

use crate::{
    context::HostApi, evaluate::NodeEvaluationError, node::Node, Language,
};

pub fn load_host_api() -> HostApi {
    HostApi::builder()
        .add_constant_endpoint("pi".into(), |_context| Node::Number {
            value: PI,
        }, HashMap::from([
            (Language::German, String::from("Die Kreiszahl π ist allgemein definiert als das Verhältnis des Umfangs eines Kreises zu seinem Durchmesser."))
        ]))
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
        }, vec![String::from("x")], 
    HashMap::from([
        (Language::German, String::from("Berechnet den Sinus von x."))
    ]))
        .build()
}
