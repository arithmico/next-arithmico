use core::f64;
use std::{cell::RefCell, collections::HashSet};

use common::Language;
use engine_derive::FunctionArguments;

use evaluator::{
    Error, ErrorKind, EvaluateNode, FunctionEndpoint, MapToEvaluatorError,
    Options,
};
use math_utils::calculate_numerical_integral;
use node::{Function, Node, Number};
use validator::{FunctionValidator, NodeValidator};
use trace::{Tracable, TracableMut};

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
        options: Options,
    ) -> Result<Self::Output, Error> {
        f.validate_one_argument()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let symbol_names = f.expression.get_symbol_names();

        let known_symbols = symbol_names
            .iter()
            .filter(|name| options.lookup(name).is_some())
            .copied()
            .collect::<HashSet<_>>();

        f.expression
            .validate_one_unknown_symbol(&known_symbols)
            .map_to_error_kind(ErrorKind::RuntimeError)?;

        let unknown_symbol = symbol_names
            .iter()
            .find(|name| !known_symbols.contains(*name))
            .copied()
            .ok_or_else(|| Error::unreachable())?;

        f.validate_argument_matches_unknown_symbol(unknown_symbol)
            .map_to_error_kind(ErrorKind::RuntimeError)?;

        let evaluation_error = RefCell::new(None::<Error>);

        let function = |x: f64| -> f64 {
            let mut local_stack = options.stack.clone();
            local_stack.insert(unknown_symbol, Number::new_node(x));

            let options = Options {
                stack: &local_stack,
                ..options
            };

            match f.expression.evaluate(options) {
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
