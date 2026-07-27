use crate::{
    Context,
    api::validations::NumberValidation,
    core::{EvaluateNodeError, FunctionEndpoint, Language},
};
use engine_derive::FunctionArguments;
use math_utils::calculate_quantile_of_binomial_cdf;
use node::Number;

#[derive(FunctionArguments)]
#[name("qbinom")]
#[description(
    Language::German,
    "Berechnet die Quantilsfunktion der Binomialverteilung. Bestimmt das kleinste k, sodass P(X <= k) >= p_q gilt, für ein binomialverteilte Zufallsvariable X."
)]
#[description(
    Language::English,
    "Computes the quantile function of the binomial distribution. Returns the smallest k such that P(X <= k) >= p_q for for a random binmoial distributed variable X."
)]
pub struct QBinomArgs<'a> {
    #[description(Language::German, "Quantil (Wahrscheinlichkeit)")]
    #[description(Language::English, "quantile (probability)")]
    p_q: &'a Number,

    #[description(Language::German, "Anzahl der Versuche")]
    #[description(Language::English, "number of trials")]
    n: &'a Number,

    #[description(Language::German, "Erfolgswahrscheinlichkeit")]
    #[description(Language::English, "success probability")]
    p: &'a Number,
}

pub struct QBinomEndpoint;

impl FunctionEndpoint for QBinomEndpoint {
    type Output = Number;
    type Arguments<'a> = QBinomArgs<'a>;

    fn executor<'a>(
        QBinomArgs { p_q, n, p }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        p_q.validate_closed_interval(0.0, 1.0)?;
        p.validate_closed_interval(0.0, 1.0)?;

        n.validate_non_negative()?.validate_integer()?;

        calculate_quantile_of_binomial_cdf(p_q.value, n.value as usize, p.value)
            .map_err(|_| EvaluateNodeError::runtime_error("qbinom"))
            .map(|result| Number::new(result as f64))
    }
}
