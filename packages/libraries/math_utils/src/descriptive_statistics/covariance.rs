use crate::calculate_average;

pub fn calculate_covariance(xs: &[f64], ys: &[f64]) -> Option<f64> {
    if xs.len() != ys.len() {
        return None;
    }

    let mean_x = calculate_average(xs)?;
    let mean_y = calculate_average(ys)?;

    let result = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| (x - mean_x) * (y - mean_y))
        .sum::<f64>()
        / (xs.len() - 1) as f64;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn covariance() {
        assert_eq!(
            calculate_covariance(
                &[18.0, 2.0, 42.0, 14.0, 22.0, 35.0, 45.0, 8.0],
                &[22.0, 10.0, 53.0, 30.0, 25.0, 36.0, 45.0, 13.0],
            ),
            Some(222.92857142857142)
        );
    }

    #[test]
    fn covariance_none() {
        assert_eq!(calculate_covariance(&[], &[],), None);
    }

    #[test]
    fn covariance_inequal_none() {
        assert_eq!(calculate_covariance(&[1.0], &[],), None);
    }
}
