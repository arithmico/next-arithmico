use std::f64::consts::PI;

use crate::{
    core::{host_api::HostApiModule, node::*},
    language::Language,
};

pub fn load_trigonometry_module() -> HostApiModule {
    HostApiModule::builder()
        .name("trigonometry")

        .endpoint(cfg!(feature = "api_endpoint_trigonometry_pi"),"pi", |builder| {
            builder
                .description(Language::English, "tbd")
                .description(Language::German, "Die Kreiszahl π ist allgemein definiert als das Verhältnis des Umfangs eines Kreises zu seinem Durchmesser.")
                .constant(|_context| Number::new(PI).into())
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
                        Node::Number(Number { value }) => {
                            if value.rem_euclid(PI).abs() < value * f64::EPSILON
                            {
                                return Ok(Number::new(0.0).into());
                            }
                            Ok(Number::new(value.sin()).into())
                        }
                        _ => Err(NodeEvaluationError::RuntimeError(
                            "invalid argument type".into(),
                        )),
                    }
                })
        })
        .build()
}
