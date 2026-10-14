use crate::{
    DistributionError,
    distributions::incomplete_gamma_function_ratio::calculate_regularized_upper_incomplete_gamma,
};

/// Computes the cumulative distribution function of the Poisson distribution.
///
/// Returns the probability `P(X <= k)` for a Poisson-distributed random
/// variable with mean `lambda`.
pub fn calculate_poisson_cdf(
    k: usize,
    lambda: f64,
) -> Result<f64, DistributionError> {
    calculate_regularized_upper_incomplete_gamma((k + 1) as f64, lambda)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poisson_cdf() {
        assert_eq!(
            calculate_poisson_cdf(5, 3.0).unwrap(),
            0.9160820579686966 // wolfram alpha: 0.9160820579686965508199004479611366900232724962669871664443501879515162862356717228345980939540535393
        );
    }
}
