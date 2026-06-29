use crate::{
    core::{EvaluateNodeError, FunctionEndpoint, Language},
    Context,
};
use engine_derive::FunctionArguments;
use float_utils::F64Extension;
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
        let p_q_val = p_q.value;
        let n_val = n.value;
        let p_val = p.value;

        if !p_q_val.is_in_closed_interval(0.0, 1.0) {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.distributions.binomial.closed_interval_zero_one",
            )
            .build()
            .with_tracable(p_q));
        }

        if !p_val.is_in_closed_interval(0.0, 1.0) {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.distributions.binomial.closed_interval_zero_one",
            )
            .build()
            .with_tracable(p));
        }

        if n_val < 0.0 {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.distributions.binomial.negative_value",
            )
            .build()
            .with_tracable(n));
        }

        if !n_val.is_integer() {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.distributions.binomial.non_integer",
            )
            .build()
            .with_tracable(n));
        }

        return if let Some(result) =
            calculate_quantile_of_binomial_cdf(p_q_val, n_val as usize, p_val)
        {
            Ok(Number::new(result as f64))
        } else {
            Err(EvaluateNodeError::runtime_error("cbinom"))
        };
    }
}
