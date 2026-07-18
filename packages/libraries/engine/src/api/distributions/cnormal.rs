use std::sync::LazyLock;

use engine_derive::FunctionArguments;
use math_utils::calculate_normal_cdf;
use node::IntoNode;
use node::Node;
use node::Number;

use crate::{
    core::{EvaluateNodeError, FunctionEndpoint, Language},
    Context,
};

static DEFAULT_MEAN: LazyLock<Node> =
    LazyLock::new(|| Number::new(0.0).into_node());

static DEFAULT_SD: LazyLock<Node> =
    LazyLock::new(|| Number::new(1.0).into_node());

#[derive(FunctionArguments)]
#[name("cnormal")]
#[description(
    Language::German,
    "Calculates the cumulative normal distribution of x. If no further parameters are passed, the standard cumulative normal distribution is calculated.")]
#[description(
    Language::English,
    "Berechnet die kumulierte Normalverteilung von x. Wenn keine weiteren Parameter übergeben werden, wird die kumulierte Standardnormalverteilung berechnet."
)]
pub struct CNormalArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    x: &'a Number,

    #[default(&*DEFAULT_MEAN)]
    #[description(Language::German, "Mittelwert")]
    #[description(Language::English, "mean")]
    mean: &'a Number,

    #[default(&*DEFAULT_SD)]
    #[description(Language::German, "Standardabweichung")]
    #[description(Language::English, "standard deviation")]
    sd: &'a Number,
}

pub struct CNormalEndpoint;

impl FunctionEndpoint for CNormalEndpoint {
    type Output = Number;
    type Arguments<'a> = CNormalArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        CNormalArgs { x, mean, sd }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let x = x.value;
        let mean = mean.value;
        let standard_deviation = sd.value;

        if standard_deviation < 0.0 {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.distributions.normal.smaller_than_zero",
            )
            .build()
            .with_tracable(sd));
        }

        return if let Ok(result) = calculate_normal_cdf(
            x,
            mean,
            standard_deviation,
        ) {
            Ok(Number::new(result))
        } else {
            Err(EvaluateNodeError::runtime_error("cnormal"))
        };
    }
}
