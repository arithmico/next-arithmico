use crate::{calculate_covariance, calculate_unbiased_standard_deviation};

pub fn calculate_correlation_coefficient(
    xs: &[f64],
    ys: &[f64],
) -> Option<f64> {
    if xs.len() != ys.len() {
        return None;
    }

    let sd_x = calculate_unbiased_standard_deviation(xs)?;
    let sd_y = calculate_unbiased_standard_deviation(ys)?;

    let denominator = sd_x * sd_y;

    if denominator == 0.0 {
        return None;
    }

    Some(calculate_covariance(xs, ys)? / denominator)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correlation_coefficient() {
        assert_eq!(
            calculate_correlation_coefficient(
                &[10.0, 8.0, 13.0, 9.0, 11.0, 14.0, 6.0, 4.0, 12.0, 7.0, 5.0],
                &[
                    8.04, 6.95, 7.58, 8.81, 8.33, 9.96, 7.24, 4.26, 10.84,
                    4.82, 5.68
                ],
            ),
            Some(0.8164205163448399)
        );
    }

    #[test]
    fn correlation_coefficient_none() {
        assert_eq!(
            calculate_correlation_coefficient(&[1.0, 1.0], &[1.0, 1.0],),
            None
        );
    }

    #[test]
    fn correlation_coefficient_inequal_none() {
        assert_eq!(calculate_correlation_coefficient(&[1.0], &[],), None);
    }
}
