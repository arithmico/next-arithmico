use core::f64;
use std::{collections::HashSet, sync::LazyLock};

use common::Language;
use engine_derive::FunctionArguments;

use evaluator::{
    Error, ErrorKind, EvaluateNode, FunctionEndpoint, MapToEvaluatorError,
    Options,
};
use math_utils::calculate_numerical_derivative;
use node::{Function, IntoNode, Node, Number};
use validator::{FunctionValidator, NodeValidator, NumberValidator};

static DEFAULT_ORDER: LazyLock<Node> =
    LazyLock::new(|| Number::new(1.0).into_node());

#[derive(FunctionArguments)]
#[name("nderive")]
#[description(
    Language::German,
    "Berechnet den Wert der Ableitungsfunktion an der gegebenen Position."
)]
#[description(
    Language::English,
    "Calculates the derivative of the function for the given position."
)]
pub struct NDeriveArgs<'a> {
    #[description(Language::German, "Funktion")]
    #[description(Language::English, "function")]
    f: &'a Function,

    #[description(Language::German, "Position")]
    #[description(Language::English, "position")]
    position: &'a Number,

    #[default(&*DEFAULT_ORDER)]
    #[description(Language::German, "Ordnung der Ableitung")]
    #[description(Language::English, "derivative order")]
    order: &'a Number,
}

pub struct NDeriveEndpoint;

impl FunctionEndpoint for NDeriveEndpoint {
    type Output = Number;
    type Arguments<'a> = NDeriveArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        NDeriveArgs { f, position, order }: Self::Arguments<'a>,
        options: Options,
    ) -> Result<Self::Output, Error> {
        order
            .validate_greater_than(0.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

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

        let mut local_stack = options.stack.clone();
        let mut function = |x: f64| -> f64 {
            local_stack.insert(unknown_symbol, Number::new(x).into_node());

            let local_options = Options {
                stack: &local_stack,
                ..options
            };

            match f.expression.evaluate(local_options) {
                Ok(Node::Number(value)) => value.value,
                _ => f64::NAN,
            }
        };

        let result = calculate_numerical_derivative(
            &mut function,
            position.value,
            order.value as usize,
        );

        Ok(Number::new(result))
    }
}
