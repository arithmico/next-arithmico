use core::f64;

use engine_derive::FunctionArguments;

use math_utils::{IntegrationError, calculate_numerical_integral};
use node::{Function, IntoNode, Node, Number};
use trace::{Tracable, TracableMut};

use crate::{
    Context,
    core::{EvaluateNode, EvaluateNodeError, FunctionEndpoint, Language},
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
    ) -> Result<Self::Output, EvaluateNodeError> {
        let arguments = f.signature.arguments();
        let arguments_count = arguments.len();

        if arguments_count > 1 {
            return Err(EvaluateNodeError::too_many_parameters(
                arguments_count,
            )
            .with_optional_span(f.hull()));
        }
        if arguments_count < 1 {
            return Err(EvaluateNodeError::missing_parameter("")
                .with_optional_span(f.hull()));
        }

        let function = |x: f64| -> f64 {
            let argument_name = &arguments[0].get_name();
            let mut local_context = context.clone();

            local_context
                .stack
                .insert(argument_name, Number::new(x).into_node());

            match f.evaluate(&local_context) {
                Ok(Node::Number(value)) => value.value,
                _ => f64::NAN,
            }
        };

        calculate_numerical_integral(&function, start.value, stop.value)
            .map(|result| Number::new(result))
            .map_err(|error| {
                EvaluateNodeError::from(error)
                    .with_optional_span(f.expression.hull())
            })
    }
}

impl From<IntegrationError> for EvaluateNodeError {
    fn from(value: IntegrationError) -> Self {
        match value {
        IntegrationError::InvalidTolerance => {
            EvaluateNodeError::generic_runtime_error(
                "engine.api.error.generic_runtime_error.nintegrate.invalid_tolerance",
            )
            .build()
        }
        IntegrationError::SubdivisionLimitReached => {
            EvaluateNodeError::generic_runtime_error(
                "engine.api.error.generic_runtime_error.nintegrate.subdivision_limit_reached",
            )
            .build()
        }
        IntegrationError::RoundoffError => {
            EvaluateNodeError::generic_runtime_error(
                "engine.api.error.generic_runtime_error.nintegrate.roundoff_error",
            )
            .build()
        }
        IntegrationError::BadIntegrandBehavior => {
            EvaluateNodeError::generic_runtime_error(
                "engine.api.error.generic_runtime_error.nintegrate.bad_integrand_behavior",
            )
            .build()
        }
        IntegrationError::ExtrapolationFailed => {
            EvaluateNodeError::generic_runtime_error(
                "engine.api.error.generic_runtime_error.nintegrate.extrapolation_failed",
            )
            .build()
        }
        IntegrationError::ProbablyDivergent => {
            EvaluateNodeError::generic_runtime_error(
                "engine.api.error.generic_runtime_error.nintegrate.probably_divergent",
            )
            .build()
        }
        IntegrationError::NonFiniteIntegrand { x, value } => {
            EvaluateNodeError::generic_runtime_error(
                "engine.api.error.generic_runtime_error.nintegrate.non_finite_integrand",
            )
            .key("x", x)
            .key("value", value)
            .build()
                }
            }
    }
}
