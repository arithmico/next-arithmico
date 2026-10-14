/// Computes the sample quantile using Hyndman and Fan's Type 7 estimator.
///
/// The input values are sorted in ascending order and the quantile is
/// obtained by linear interpolation between adjacent order statistics.
///
/// For a sample of size `n` and probability `p`, Type 7 uses the
/// zero-based fractional index
///
/// `h = (n - 1) * p`.
///
/// If `h = h_floot + g`, where `h_floor = floor(h)` and `0 <= g < 1`, the quantile is
///
/// `Q(p) = x[i] + g * (x[h_floor + 1] - x[h_floor])`,
///
/// where `x` denotes the sorted sample. At the upper boundary, the last
/// sample value is returned.
///
/// This corresponds to Type 7 in the classification of Hyndman and Fan
/// and is the default sample quantile estimator used by R's `quantile`
/// function.
///
/// # Arguments
///
/// * `p` - Probability for which the quantile is calculated, expected to
///   be in the closed interval `[0, 1]`.
/// * `values` - Sample values from which the quantile is calculated.
///
/// # References
///
/// * R. J. Hyndman and Y. Fan,
///   "Sample Quantiles in Statistical Packages",
///   The American Statistician, 50(4), pp. 361-365, 1996.
///   <https://doi.org/10.2307/2684934>
///
/// * R `stats::quantile` documentation:
///   <https://stat.ethz.ch/R-manual/R-devel/library/stats/html/quantile.html>
///
/// # Implementation
/// This implementation relies on the table of the following Wikipedia page
/// (attention the h values are 1-indexed, this implementation is 0-indexed):
/// <https://en.wikipedia.org/wiki/Quantile#Estimating_quantiles_from_a_sample>
pub fn calculate_sample_quantile(p: f64, values: &mut [f64]) -> f64 {
    values.sort_by(|a, b| a.total_cmp(b));

    let h = (values.len() - 1) as f64 * p;
    let h_floor = h.floor() as usize;
    let fraction = h - h_floor as f64;

    if h_floor + 1 < values.len() {
        values[h_floor] + fraction * (values[h_floor + 1] - values[h_floor])
    } else {
        values[h_floor]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantile_01() {
        assert_eq!(
            calculate_sample_quantile(
                0.25,
                &mut [6.0, 1.0, 3.0, 8.0, 4.0, 5.0, 2.0, 7.0, 9.0]
            ),
            3.0, // type 6: 2.5
        );
    }

    #[test]
    fn quantile_02() {
        assert_eq!(
            calculate_sample_quantile(
                0.75,
                &mut [1.0, 2.0, 2.0, 3.0, 5.0, 8.0, 9.0, 12.0, 12.0, 13.0]
            ),
            11.25, // type 6: 12.0
        );
    }
}
