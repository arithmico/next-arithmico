use std::sync::LazyLock;

use common::Language;
use engine_derive::FunctionArguments;
use evaluator::Error;
use evaluator::ErrorKind;
use evaluator::FunctionEndpoint;
use evaluator::MapToEvaluatorError;
use evaluator::Options;
use math_utils::calculate_normal_pdf;
use node::IntoNode;
use node::Node;
use node::Number;
use validator::NumberValidator;

static DEFAULT_MEAN: LazyLock<Node> =
    LazyLock::new(|| Number::new(0.0).into_node());

static DEFAULT_SD: LazyLock<Node> =
    LazyLock::new(|| Number::new(1.0).into_node());

#[derive(FunctionArguments)]
#[name("normal")]
#[description(
    Language::German,
    "Berechnet die Normalverteilung von x. Falls keine weiteren Parameter übergeben werden, wird die Standardnormalverteilung berechnet."
)]
#[description(
    Language::English,
    "Calculates the normal distribution of x. If no further parameters are passed, the standard normal distribution is calculated."
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
        _context: Options,
    ) -> Result<Self::Output, Error> {
        sd.validate_greater_than(0.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        calculate_normal_pdf(x.value, mean.value, sd.value)
            .map_err(|_| Error::runtime_error("normal"))
            .map(|result| Number::new(result))
    }
}
