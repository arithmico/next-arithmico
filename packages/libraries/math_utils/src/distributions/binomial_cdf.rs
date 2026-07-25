use crate::{
    distributions::incomplete_beta_function::calculate_complementary_incomplete_beta_function,
    DistributionError,
};

/// Computes the cumulative distribution function of the binomial distribution.
///
/// Returns
///
/// `P(X <= k)`
///
/// for a binomially distributed random variable
///
/// `X ~ Binomial(n, p)`.
///
/// The implementation uses the identity
///
/// `P(X <= k) = 1 - I_p(k + 1, n - k)`,
///
/// where `I_x(a, b)` is the regularized incomplete beta function.
///
/// # Errors
///
/// Returns `Err` if `k > n` or if the underlying incomplete
/// beta function rejects the parameters.
pub fn calculate_binomial_cdf(
    n: usize,
    p: f64,
    k: usize,
) -> Result<f64, DistributionError> {
    if !(0.0..=1.0).contains(&p) {
        return Err(DistributionError::OutOfRange {
            min: 0.0,
            max: 1.0,
            actual: p,
        });
    }

    if k >= n || n == 0 {
        return Ok(1.0);
    }

    calculate_complementary_incomplete_beta_function(
        p,
        (k + 1) as f64,
        (n - k) as f64,
    )
    .map_err(|_| DistributionError::BetaError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binomial_cdf_01_n_zero_k_zero() {
        assert_eq!(calculate_binomial_cdf(0, 0.7, 0).unwrap(), 1.0);
    }

    #[test]
    fn binomial_cdf_02_n_10_p_0_87_k_9() {
        assert_eq!(
            calculate_binomial_cdf(10, 0.87, 9).unwrap(),
            0.7515765858085643
        );
    }

    #[test]
    fn binomial_cdf_03_n_5000_p_0_2_k_999() {
        assert_eq!(
            calculate_binomial_cdf(5000, 0.2, 999).unwrap(),
            0.4943582593585349
        );
    }

    #[test]
    fn binomial_cdf_04_k_equals_n() {
        assert_eq!(calculate_binomial_cdf(10, 0.3, 10).unwrap(), 1.0);
    }

    #[test]
    fn binomial_cdf_05_k_zero() {
        assert_eq!(
            calculate_binomial_cdf(5, 0.2, 0).unwrap(),
            0.32768000000000014
        );
    }

    #[test]
    fn binomial_cdf_06_fair_coin_n_10_k_3() {
        assert_eq!(
            calculate_binomial_cdf(10, 0.5, 3).unwrap(),
            0.17187500000000003
        );
    }
}
