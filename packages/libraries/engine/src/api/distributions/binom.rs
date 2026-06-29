use crate::{
    core::{EvaluateNodeError, FunctionEndpoint, Language},
    Context,
};
use engine_derive::FunctionArguments;
use float_utils::F64Extension;
use math_utils::calculate_binomial_pmf;
use node::Number;

#[derive(FunctionArguments)]
#[name("binom")]
#[description(
    Language::German,
    "Berechnet die Wahrscheinlichkeitsfunktion der Binomialverteilung."
)]
#[description(
    Language::English,
    "Computes the probability mass function of the binomial distribution."
)]
pub struct BinomArgs<'a> {
    #[description(Language::German, "Anzahl der Versuche")]
    #[description(Language::English, "number of trials")]
    n: &'a Number,

    #[description(Language::German, "Erfolgswahrscheinlichkeit")]
    #[description(Language::English, "success probability")]
    p: &'a Number,

    #[description(Language::German, "Anzahl der Erfolge")]
    #[description(Language::English, "number of successes")]
    k: &'a Number,
}

pub struct BinomEndpoint;

impl FunctionEndpoint for BinomEndpoint {
    type Output = Number;
    type Arguments<'a> = BinomArgs<'a>;

    fn executor<'a>(
        BinomArgs { n, p, k }: Self::Arguments<'a>,
        _context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError> {
        let n_val = n.value;
        let p_val = p.value;
        let k_val = k.value;

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
        if k_val < 0.0 {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.distributions.binomial.negative_value",
            )
            .build()
            .with_tracable(k));
        }

        if !n_val.is_integer() {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.distributions.binomial.non_integer",
            )
            .build()
            .with_tracable(n));
        }
        if !k_val.is_integer() {
            return Err(EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.distributions.binomial.non_integer",
            )
            .build()
            .with_tracable(k));
        }

        if k_val > n_val {
            return Ok(Number::new(0.0));
        }

        let result = calculate_binomial_pmf(n_val as usize, p_val, k_val as usize);

        Ok(Number::new(result))
    }
}
