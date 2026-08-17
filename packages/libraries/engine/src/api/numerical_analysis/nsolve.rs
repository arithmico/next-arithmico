use core::f64;
use std::{collections::HashSet, sync::LazyLock};

use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, EvaluateNode, FunctionEndpoint, MapToEvaluatorError,
    Options,
};
use math_utils::find_roots;
use node::{Equals, IntoNode, Negate, Node, Number, Sum, Tensor};
use validator::{NodeValidator, NumberValidator};

static DEFAULT_START: LazyLock<Node> =
    LazyLock::new(|| Number::new(-20.0).into_node());
static DEFAULT_STOP: LazyLock<Node> =
    LazyLock::new(|| Number::new(20.0).into_node());

#[derive(FunctionArguments)]
#[name("nsolve")]
#[description(
    Language::German,
    "Sucht numerisch nach Lösungen für die gegebene Gleichung in den Grenzen start und stop."
)]
#[description(
    Language::English,
    "Searches numerically for solutions to the given equation within the start and stop limits."
)]
pub struct NSolveArgs<'a> {
    #[skip_evaluate]
    #[description(Language::German, "Zu lösende Gleichung")]
    #[description(Language::English, "Equation to be solved")]
    equation: &'a Equals,

    #[default(&*DEFAULT_START)]
    #[description(Language::German, "Startwert")]
    #[description(Language::English, "start value")]
    start: &'a Number,

    #[default(&*DEFAULT_STOP)]
    #[description(Language::German, "Endwert")]
    #[description(Language::English, "stop value")]
    stop: &'a Number,
}

pub struct NSolveEndpoint;

impl FunctionEndpoint for NSolveEndpoint {
    type Output = Tensor;
    type Arguments<'a> = NSolveArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        NSolveArgs {
            equation,
            start,
            stop,
        }: Self::Arguments<'a>,
        options: Options,
    ) -> Result<Self::Output, Error> {
        start
            .validate_less_than(stop.value)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let expression = Sum::new(vec![
            *equation.left.clone(),
            Negate::new(*equation.right.clone()),
        ]);

        let symbol_names = expression.get_symbol_names();

        let known_symbols = symbol_names
            .iter()
            .filter(|name| options.lookup(name).is_some())
            .copied()
            .collect::<HashSet<_>>();

        expression
            .validate_one_unknown_symbol(&known_symbols)
            .map_to_error_kind(ErrorKind::RuntimeError)?;

        let variable_name = symbol_names
            .iter()
            .find(|name| !known_symbols.contains(*name))
            .ok_or_else(|| Error::unreachable())?;

        let function = |x: f64| -> Option<f64> {
            let mut local_stack = options.stack.clone();
            local_stack.insert(variable_name, Number::new(x).into_node());
            let options = Options {
                stack: &local_stack,
                ..options
            };

            match expression.evaluate(options) {
                Ok(Node::Number(value)) if value.value.is_finite() => {
                    Some(value.value)
                }
                _ => None,
            }
        };

        let roots = find_roots(&function, start.value, stop.value)
            .map_to_error_kind(ErrorKind::RuntimeError)?;

        let results = roots
            .iter()
            .map(|root| Number::new(*root).into_node())
            .collect();

        Ok(Tensor::new(results))
    }
}
