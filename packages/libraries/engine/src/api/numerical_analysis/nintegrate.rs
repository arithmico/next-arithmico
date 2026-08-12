use core::f64;
use std::cell::RefCell;

use engine_derive::FunctionArguments;

use evaluator::{Error, ErrorKind, MapToEvaluatorError};
use math_utils::calculate_numerical_integral;
use node::{Function, IntoNode, Node, Number};
use trace::{Tracable, TracableMut};

use crate::{
    Context,
    core::{EvaluateNode, FunctionEndpoint, Language},
};

#[derive(FunctionArguments)]
#[name("nintegrate")]
#[description(
    Language::German,
    "Berechnet das bestimmte Integral von f zwischen start und stop."
)]
#[description(
    Language::English,
    "Calculates the definite integral of f between start and stop."
)]
pub struct NIntegrateArgs<'a> {
    #[description(Language::German, "Funktion")]
    #[description(Language::English, "function")]
    f: &'a Function,

    #[description(Language::German, "Startwert")]
    #[description(Language::English, "start value")]
    start: &'a Number,

    #[description(Language::German, "Endwert")]
    #[description(Language::English, "stop value")]
    stop: &'a Number,
}

pub struct NIntegrateEndpoint;

impl FunctionEndpoint for NIntegrateEndpoint {
    type Output = Number;
    type Arguments<'a> = NIntegrateArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        NIntegrateArgs { f, start, stop }: Self::Arguments<'a>,
        context: &Context,
    ) -> Result<Self::Output, Error> {
        let arguments = f.signature.arguments();
        let arguments_count = arguments.len();

        if arguments_count > 1 {
            return Err(Error::too_many_parameters(arguments_count)
                .with_optional_span(f.hull()));
        }
        if arguments_count < 1 {
            return Err(
                Error::missing_parameter("").with_optional_span(f.hull())
            );
        }

        let evaluation_error = RefCell::new(None::<Error>);

        let function = |x: f64| -> f64 {
            let argument_name = &arguments[0].get_name();
            let mut local_context = context.clone();

            local_context
                .stack
                .insert(argument_name, Number::new(x).into_node());

            match f.expression.evaluate(&local_context) {
                Ok(Node::Number(value)) => value.value,
                Err(error) => {
                    if evaluation_error.borrow().is_none() {
                        *evaluation_error.borrow_mut() = Some(error);
                    }

                    f64::NAN
                }
                Ok(_) => f64::NAN,
            }
        };

        let result =
            calculate_numerical_integral(&function, start.value, stop.value);

        if let Some(error) = evaluation_error.into_inner() {
            return Err(error.with_optional_span(f.hull()));
        }

        result
            .map_to_error_kind(ErrorKind::RuntimeError)
            .map(|result| Number::new(result))
    }
}
