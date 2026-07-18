use std::sync::LazyLock;

use engine_derive::FunctionArguments;
use float_utils::F64Extension;
use math_utils::calculate_quantile_of_normal_cdf;
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
#[name("qnormal")]
#[description(
    Language::German,
    "Quantilsfunktion der kumulierten Normalverteilung. Berechnet die entsprechende Zufallsvariable für das gegebene Quantil p."
)]
#[description(
    Language::English,
    "Quantile function of the cumulative normal distribution. Calculate the corresponding random variable for the given quantile p."
)]
pub struct QNormalArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    p: &'a Number,

    #[default(&*DEFAULT_MEAN)]
    #[description(Language::German, "Mittelwert")]
    #[description(Language::English, "mean")]
    mean: &'a Number,

    #[default(&*DEFAULT_SD)]
    #[description(Language::German, "Standardabweichung")]
    #[description(Language::English, "standard deviation")]
    sd: &'a Number,
}

pub struct QNormalEndpoint;

impl FunctionEndpoint for QNormalEndpoint {
    type Output = Number;
    type Arguments<'a> = QNormalArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        QNormalArgs { p, mean, sd }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let p = p.value;
        let mean = mean.value;
        let standard_deviation = sd.value;

        if !p.is_in_open_interval(0., 1.) {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.distributions.open_interval.zero_one",
            )
            .build()
            .with_tracable(sd));
        }
        
        if standard_deviation < 0.0 {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.distributions.normal.smaller_than_zero",
            )
            .build()
            .with_tracable(sd));
        }

        return if let Ok(result) =
            calculate_quantile_of_normal_cdf(
            p,
            mean,
            standard_deviation,
        )
        {
            Ok(Number::new(result as f64))
        } else {
            Err(EvaluateNodeError::runtime_error("qnormal"))
        };
    }
}
