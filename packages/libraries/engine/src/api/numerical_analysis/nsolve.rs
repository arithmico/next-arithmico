use core::f64;
use std::sync::LazyLock;

use engine_derive::FunctionArguments;
use math_utils::{FindRootsError, find_roots};
use node::{Equals, IntoNode, Negate, Node, Number, Sum, Tensor};
use node_validator::NumberValidator;
use trace::{Tracable, TracableMut, Trace};

use crate::{
    Context,
    core::{EvaluateNode, EvaluateNodeError, FunctionEndpoint, Language},
};

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
        context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        start.validate_less_than(stop.value)?;

        let expression = Sum::new(vec![
            *equation.left.clone(),
            Negate::new(*equation.right.clone()),
        ]);

        let variable_names = expression
            .get_symbol_names()
            .into_iter()
            .filter(|name| context.lookup(name).is_none())
            .collect::<Vec<String>>();

        let variable_count = variable_names.len();
        if variable_count > 1 {
            return Err(EvaluateNodeError::too_many_parameters(variable_count)
                .with_optional_span(equation.hull()));
        }
        if variable_count < 1 {
            return Err(EvaluateNodeError::missing_parameter("")
                .with_optional_span(equation.hull()));
        }

        let function = |x: f64| -> f64 {
            let variable_name = &variable_names[0];
            let mut local_context = context.clone();

            local_context
                .stack
                .insert(variable_name, Number::new(x).into_node());

            match expression.evaluate(&local_context) {
                Ok(Node::Number(value)) => value.value,
                _ => f64::NAN,
            }
        };

        let roots = find_roots(&function, start.value, stop.value).map_err(
            |error| {
                EvaluateNodeError::from(error)
                    .with_optional_span(equation.hull())
            },
        )?;

        let results = roots
            .iter()
            .map(|root| Number::new(*root).into_node())
            .collect::<Vec<_>>();

        Ok(Tensor {
            shape: vec![results.len()],
            elements: results,
            trace: Trace::new(),
        })
    }
}

impl From<FindRootsError> for EvaluateNodeError {
    fn from(value: FindRootsError) -> Self {
        match value {
            FindRootsError::NonFiniteBoundary { name, value } => {
                EvaluateNodeError::generic_runtime_error(
                    "engine.api.error.generic_runtime_error.find_roots.non_finite_boundary",
                )
                .key("name", name)
                .key("value", value)
                .build()
            }
            FindRootsError::InvalidInterval { start, end } => {
                EvaluateNodeError::generic_runtime_error(
                    "engine.api.error.generic_runtime_error.find_roots.invalid_interval",
                )
                .key("start", start)
                .key("end", end)
                .build()
            }
            FindRootsError::NonFiniteFunctionValue { x, value } => {
                EvaluateNodeError::generic_runtime_error(
                    "engine.api.error.generic_runtime_error.find_roots.non_finite_function_value",
                )
                .key("x", x)
                .key("value", value)
                .build()
            }
            FindRootsError::EncloseZero { a, b, .. } => {
                EvaluateNodeError::generic_runtime_error(
                    "engine.api.error.generic_runtime_error.find_roots.enclose_zero",
                )
                .key("a", a)
                .key("b", b)
                .build()
            }
        }
    }
}
