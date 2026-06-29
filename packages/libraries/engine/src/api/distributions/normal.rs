use std::sync::LazyLock;

use engine_derive::FunctionArguments;
use node::IntoNode;
use node::Node;
use node::Number;

use crate::{
    api::distributions::utils::normal_utils::calculate_normal_pdf,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
    Context,
};

static DEFAULT_MEAN: LazyLock<Node> =
    LazyLock::new(|| Number::new(0.0).into_node());

static DEFAULT_SD: LazyLock<Node> =
    LazyLock::new(|| Number::new(1.0).into_node());

#[derive(FunctionArguments)]
#[name("normal")]
#[description(Language::German, "Berechnet den Cosinus von x.")]
#[description(
    Language::English,
    "Calculates the normal distribution. The default values are: for expactation 0 and for sd 1 (standard normal distribution).)"
)]
pub struct NormalArgs<'a> {
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

pub struct NormalEndpoint;

impl FunctionEndpoint for NormalEndpoint {
    type Output = Number;
    type Arguments<'a> = NormalArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        NormalArgs { x, mean, sd }: Self::Arguments<'a>,
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

        Ok(Number::new(calculate_normal_pdf(
            x,
            mean,
            standard_deviation,
        )))
    }
}
