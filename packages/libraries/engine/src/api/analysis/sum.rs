use std::sync::LazyLock;

use common::Language;
use engine_derive::FunctionArguments;

use evaluator::{
    Error, ErrorKind, EvaluateNode, FunctionEndpoint, MapToEvaluatorError,
    Options,
};
use node::{Function, Node, Number, Sum};
use validator::{FunctionValidator, NodeValidator, NumberValidator};

static DEFAULT_STEP_SIZE: LazyLock<Node> =
    LazyLock::new(|| Number::new_node(1.0));

#[derive(FunctionArguments)]
#[name("sum")]
#[description(
    Language::German,
    "Berechnet die Summe einer Funktion über einen Zahlenbereich."
)]
#[description(
    Language::English,
    "Computes the sum of evaluating a function over a range of numbers."
)]
pub struct SumArgs<'a> {
    #[description(
        Language::German,
        "Die Zielfunktion, die für jede Zahl im berechneten Bereich ausgewertet wird."
    )]
    #[description(
        Language::English,
        "The target function evaluated for each number in the calculated range."
    )]
    f: &'a Function,

    #[description(
        Language::German,
        "Die untere (inklusive) Grenze des Zahlenbereichs."
    )]
    #[description(
        Language::English,
        "The lower bound (inclusive) of the number range."
    )]
    start: &'a Number,

    #[description(
        Language::German,
        "Die obere (inklusive) Grenze des Zahlenbereichs."
    )]
    #[description(
        Language::English,
        "The upper bound (inclusive) of the number range."
    )]
    stop: &'a Number,

    #[default(&*DEFAULT_STEP_SIZE)]
    #[description(
        Language::German,
        "Optionale Schrittweite zur Inkrementierung durch den Bereich (Standardwert ist 1, falls nicht angegeben)."
    )]
    #[description(
        Language::English,
        "Optional step size used to increment through the range (defaults to 1 if omitted)."
    )]
    step: &'a Number,
}

pub struct SumEndpoint;

impl FunctionEndpoint for SumEndpoint {
    type Output = Node;
    type Arguments<'a> = SumArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        SumArgs {
            f,
            start,
            stop,
            step,
        }: Self::Arguments<'a>,
        options: Options,
    ) -> Result<Self::Output, Error> {
        f.validate_argument_count(1)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let variable_name = f
            .signature
            .argument_names()
            .first()
            .copied()
            .ok_or_else(Error::unreachable)?;

        f.expression
            .validate_unknown_symbol_name(&options.names(), variable_name)
            .map_to_error_kind(ErrorKind::RuntimeError)?;

        stop.validate_greater_than(start.value)
            .map_to_error_kind(ErrorKind::RuntimeError)?;

        step.validate_greater_than(0.0)
            .map_to_error_kind(ErrorKind::RuntimeError)?;

        let step: f64 = step.value;
        let n = (start.value - stop.value).abs().div_euclid(step) as usize + 1;
        let mut current = start.value;
        let mut stack = options.stack.clone();
        let mut elements = Vec::with_capacity(n);

        while current <= stop.value {
            let x = Number::new_node(current);
            stack.insert(variable_name, x.clone());
            let y = f.expression.evaluate(Options {
                stack: &stack,
                ..options
            })?;
            elements.push(y);
            current += step;
        }

        if elements.is_empty() {
            return Ok(Number::new_node(0.0));
        }

        match TryInto::<[Node; 1]>::try_into(elements) {
            Ok([node]) => Ok(node),
            Err(elements) => Sum::new(elements).evaluate(options),
        }
    }
}
