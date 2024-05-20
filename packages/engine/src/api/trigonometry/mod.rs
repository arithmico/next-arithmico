use std::f64::consts::PI;

use crate::{
    context::HostApiModule, evaluate::NodeEvaluationError, node::Node, Language,
};

pub fn load_trigonometry_module() -> HostApiModule {
    HostApiModule::builder()
        .name("trigonometry")
        
        .endpoint(cfg!(feature = "api_endpoint_trigonometry_pi"),"pi", |builder| {
            builder
                .description(Language::English, "tbd")
                .description(Language::German, "Die Kreiszahl π ist allgemein definiert als das Verhältnis des Umfangs eines Kreises zu seinem Durchmesser.")
                .constant(|_context| Node::Number { value: PI })
        })
        
        .endpoint(cfg!(feature = "api_endpoint_trigonometry_sin"),"sin", |builder| {
            builder
                .description(Language::English, "tbd")
                .description(Language::German, "Berechnet den Sinus von x.")
                .function(vec!["x"])
                .executor(|arguments, context| {
                    if arguments.len() != 1 {
                        return Err(NodeEvaluationError::RuntimeError(
                            "invalid number of arguments".into(),
                        ));
                    }
                    let argument = arguments.get(0).unwrap();
                    let evaluated_argument = argument.evaluate(context)?;
                    match evaluated_argument {
                        Node::Number { value } => {
                            if value.rem_euclid(PI).abs() < value * f64::EPSILON
                            {
                                return Ok(Node::Number { value: 0.0 });
                            }
                            Ok(Node::Number { value: value.sin() })
                        }
                        _ => Err(NodeEvaluationError::RuntimeError(
                            "invalid argument type".into(),
                        )),
                    }
                })
        })
        .build()
}
