use std::sync::LazyLock;

use engine_derive::FunctionArguments;
use math_utils::calculate_normal_pdf;
use node::IntoNode;
use node::Node;
use node::Number;

use crate::api::validations::NumberValidation;
use crate::{
    Context,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
};

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
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        sd.validate_non_negative()?.validate_non_zero()?;

        calculate_normal_pdf(x.value, mean.value, sd.value)
            .map_err(|_| EvaluateNodeError::runtime_error("normal"))
            .map(|result| Number::new(result))
    }
}
