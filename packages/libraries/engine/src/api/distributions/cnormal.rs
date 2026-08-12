use std::sync::LazyLock;

use engine_derive::FunctionArguments;
use evaluator::Error;
use evaluator::ErrorKind;
use evaluator::MapToEvaluatorError;
use math_utils::calculate_normal_cdf;
use node::IntoNode;
use node::Node;
use node::Number;
use node_validator::NumberValidator;

use crate::{
    Context,
    core::{FunctionEndpoint, Language},
};

static DEFAULT_MEAN: LazyLock<Node> =
    LazyLock::new(|| Number::new(0.0).into_node());

static DEFAULT_SD: LazyLock<Node> =
    LazyLock::new(|| Number::new(1.0).into_node());

#[derive(FunctionArguments)]
#[name("cnormal")]
#[description(
    Language::German,
    "Calculates the cumulative normal distribution of x. If no further parameters are passed, the standard cumulative normal distribution is calculated."
)]
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
    ) -> Result<Self::Output, Error> {
        sd.validate_greater_than(0.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        calculate_normal_cdf(x.value, mean.value, sd.value)
            .map_err(|_| Error::runtime_error("cnormal"))
            .map(|result| Number::new(result))
    }
}
