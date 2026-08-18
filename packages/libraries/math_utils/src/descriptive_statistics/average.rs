pub fn calculate_average(values: &[f64]) -> Option<f64> {
    if values.len() == 0 {
        return None;
    }

    let result = values.iter().sum::<f64>() / values.len() as f64;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average() {
        assert_eq!(
            calculate_average(&[
                10.0, 8.0, 13.0, 9.0, 11.0, 14.0, 6.0, 4.0, 12.0, 7.0, 5.0,
            ]),
            Some(9.0)
        );
    }

    #[test]
    fn average_none() {
        assert_eq!(calculate_average(&[]), None);
    }
}
