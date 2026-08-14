use std::sync::LazyLock;

use common::Language;
use engine_derive::FunctionArguments;
use evaluator::Error;
use evaluator::ErrorKind;
use evaluator::FunctionEndpoint;
use evaluator::MapToEvaluatorError;
use evaluator::Options;
use math_utils::calculate_quantile_of_normal_cdf;
use node::IntoNode;
use node::Node;
use node::Number;
use node_validator::NumberValidator;

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
        _context: Options,
    ) -> Result<Self::Output, Error> {
        p.validate_inside_open_interval(0.0, 1.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;
        sd.validate_greater_than(0.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        calculate_quantile_of_normal_cdf(p.value, mean.value, sd.value)
            .map_err(|_| Error::runtime_error("qnormal"))
            .map(|result| Number::new(result))
    }
}
