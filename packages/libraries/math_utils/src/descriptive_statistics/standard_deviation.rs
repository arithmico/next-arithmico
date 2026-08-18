use crate::{calculate_biased_variance, calculate_unbiased_variance};

pub fn calculate_biased_standard_deviation(values: &[f64]) -> Option<f64> {
    if values.len() == 0 {
        return None;
    }

    Some(calculate_biased_variance(values)?.sqrt())
}

pub fn calculate_unbiased_standard_deviation(values: &[f64]) -> Option<f64> {
    if values.len() == 0 {
        return None;
    }

    Some(calculate_unbiased_variance(values)?.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biased_standard_deviation() {
        assert_eq!(
            calculate_biased_standard_deviation(&[
                10.0, 8.0, 13.0, 9.0, 11.0, 14.0, 6.0, 4.0, 12.0, 7.0, 5.0,
            ]),
            Some(10.0_f64.sqrt())
        );
    }

    #[test]
    fn biased_standard_deviation_none() {
        assert_eq!(calculate_biased_standard_deviation(&[]), None);
    }

    #[test]
    fn unbiased_standard_deviation() {
        assert_eq!(
            calculate_unbiased_standard_deviation(&[
                10.0, 8.0, 13.0, 9.0, 11.0, 14.0, 6.0, 4.0, 12.0, 7.0, 5.0,
            ]),
            Some(11.0_f64.sqrt())
        );
    }

    #[test]
    fn unbiased_standard_deviation_none() {
        assert_eq!(calculate_unbiased_standard_deviation(&[]), None);
    }
}
