use crate::calculate_average;

pub fn calculate_biased_variance(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let average = calculate_average(values)?;

    let result = values
        .iter()
        .map(|number| (number - average).powi(2))
        .sum::<f64>()
        / values.len() as f64;

    Some(result)
}

pub fn calculate_unbiased_variance(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let average = calculate_average(values)?;

    let result = values
        .iter()
        .map(|number| (number - average).powi(2))
        .sum::<f64>()
        / (values.len() - 1) as f64;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biased_variance() {
        assert_eq!(
            calculate_biased_variance(&[
                10.0, 8.0, 13.0, 9.0, 11.0, 14.0, 6.0, 4.0, 12.0, 7.0, 5.0,
            ]),
            Some(10.0)
        );
    }

    #[test]
    fn biased_variance_none() {
        assert_eq!(calculate_biased_variance(&[]), None);
    }

    #[test]
    fn unbiased_variance() {
        assert_eq!(
            calculate_unbiased_variance(&[
                10.0, 8.0, 13.0, 9.0, 11.0, 14.0, 6.0, 4.0, 12.0, 7.0, 5.0,
            ]),
            Some(11.0)
        );
    }

    #[test]
    fn unbiased_variance_none() {
        assert_eq!(calculate_unbiased_variance(&[]), None);
    }
}
