use crate::{
    DistributionError, calculate_binomial_cdf, calculate_quantile_of_normal_cdf,
};

/// Computes the quantile (inverse CDF) of the binomial distribution.
///
/// Returns the smallest integer `k` such that
///
/// ```text
/// P(X <= k) => p_q
/// ```
///
/// for a binomially distributed random variable
///
/// ```text
/// X ~ Binomial(n, p)
/// ```
///
/// # Arguments
///
/// * `p_q` - Target probability (0 <= `p_q` <= 1)
/// * `n`   - Number of trials (n >= 0)
/// * `p`   - Success probability (0 <= `p` <= 1)
///
/// # Returns
///
/// * `Ok(k)` where `k` is the binomial quantile
/// * `Err(...)` if `p_q` is outside `[0, 1]`
///
/// # Edge Cases
///
/// * `p_q == 0` → returns `0`
/// * `p_q == 1` → returns `n`
/// * `n == 0`   → returns `0`
/// * Degenerate distributions (`p == 0` or `p == 1`) are handled
///   via the variance check (`σ == 0`)
///
/// # Notes
///
/// * The implementation is numerically stable and suitable for large `n`.
/// * The discrete refinement guarantees correctness even when the
///   normal approximation is inaccurate.
/// * This function is equivalent in behavior to R's `qbinom`.
///
/// # References
///
/// * R source: https://github.com/wch/r-source/blob/trunk/src/nmath/qbinom.c
/// * R discrete search: https://github.com/wch/r-source/blob/trunk/src/nmath/qDiscrete_search.h
/// * Cornish–Fisher expansion (normal approximation theory)
pub fn calculate_quantile_of_binomial_cdf(
    p_q: f64,
    n: usize,
    p: f64,
) -> Result<usize, DistributionError> {
    if p_q < 0.0 || p_q > 1.0 {
        return Err(DistributionError::OutOfRange {
            min: 0.0,
            max: 1.0,
            actual: p_q,
        });
    }

    if p_q == 0.0 || n == 0 {
        return Ok(0);
    }
    if p_q == 1.0 {
        return Ok(n);
    }

    qbinom(p_q, n, p)
}

/// This implementation follows the approach used in the R math library (`qbinom`)
/// and is based on:
///
/// - `qbinom.c` (R source)
/// - `qDiscrete_search.h` (discrete search refinement)
///
/// The computation proceeds in three stages:
///
/// 1. **Initial approximation (Cornish–Fisher expansion)**
///    - Uses a normal approximation:
///      ```text
///      k ≈ μ + σ z + correction
///      ```
///    - where `z` is the standard normal quantile.
///    - Includes skewness correction for improved accuracy.
///
/// 2. **Rounding**
///    - The approximation is converted to an integer using:
///      ```text
///      floor(y + 0.5)
///      ```
///    - This matches the behavior of the original C implementation.
///
/// 3. **Discrete search refinement**
///    - Uses an exponential step search ("step doubling") followed by
///      a linear correction to ensure:
///      ```text
///      F(k-1) < p_q ≤ F(k)
///      ```
///
fn qbinom(p_q: f64, n: usize, p: f64) -> Result<usize, DistributionError> {
    // from qbinom.c
    // (NB: unavoidable cancellation for pr ~= 1)
    let n_f = n as f64;

    let q = 1.0 - p;
    let mu = n_f * p;
    let sigma = (n_f * p * q).sqrt();

    if sigma == 0.0 {
        return Ok(mu.round() as usize);
    }

    let gamma = (q - p) / sigma;

    // from qDiscrete_search.h
    /* y := approx.value (Cornish-Fisher expansion) :  */
    let z = calculate_quantile_of_normal_cdf(p_q, 0.0, 1.0)?;

    let correction = gamma * (z * z - 1.0) / 6.0;
    let mut y = mu + sigma * (z + correction);

    if y < 0.0 {
        y = 0.0;
    }

    /* Algorithmic "tuning parameters", used to be hardwired; changed for speed &| precision */
    let mut k = (y + 0.5).floor() as usize; // = y.round

    if k > n {
        k = n;
    }

    do_search(k, n, p, p_q)
}

fn do_search(
    k: usize,
    n: usize,
    p: f64,
    p_q: f64,
) -> Result<usize, DistributionError> {
    // equals: y = F(k)
    let z = calculate_binomial_cdf(n, p, k)?;

    let mut incr = 1;
    let mut k = k;

    if z < p_q {
        // upward search
        let mut k_new = k + incr;
        while k_new < n {
            let z_new = calculate_binomial_cdf(n, p, k_new)?;

            if z_new >= p_q {
                k = k_new;
                break;
            }

            k = k_new;
            incr *= 2;
            k_new = k + incr;
        }

        if k + incr >= n {
            k = n;
        }
    } else {
        // downward search
        let mut k_new = k - incr;
        while k > 0 {
            let z_new = calculate_binomial_cdf(n, p, k_new)?;

            if z_new < p_q {
                k = k_new;
                break;
            }

            k = k_new;
            incr *= 2;
            k_new = k - incr;
        }
    }

    // final linear search
    // ensure: F(k-1) < p_q <= F(k)
    while k < n {
        let z = calculate_binomial_cdf(n, p, k)?;
        if z >= p_q {
            break;
        }
        k += 1;
    }

    while k > 0 {
        let z_prev = calculate_binomial_cdf(n, p, k - 1)?;
        if z_prev < p_q {
            break;
        }
        k -= 1;
    }

    Ok(k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantile_binomial_cdf_p_q_zero() {
        assert_eq!(calculate_quantile_of_binomial_cdf(0.0, 4, 0.4).unwrap(), 0);
    }

    #[test]
    fn quantile_binomial_cdf_n_zero() {
        assert_eq!(calculate_quantile_of_binomial_cdf(0.1, 0, 0.4).unwrap(), 0);
    }

    #[test]
    fn quantile_binomial_cdf_01_basic() {
        assert_eq!(
            calculate_quantile_of_binomial_cdf(0.25, 10, 1.0 / 3.0).unwrap(),
            2
        );
    }

    #[test]
    fn quantile_binomial_cdf_02_extreme_p() {
        assert_eq!(
            calculate_quantile_of_binomial_cdf(0.4, 23, 0.99).unwrap(),
            23
        );
    }
}
