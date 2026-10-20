use rand::random_range;

pub fn random_float() -> f64 {
    random_range(0.0..1.0)
}

pub fn random_integer(min: i64, max: i64) -> i64 {
    random_range(min..=max)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_random_float() {
        let number = random_float();

        assert!((0.0..1.0).contains(&number))
    }

    #[test]
    fn test_random_integer() {
        let min = -4;
        let max = 8;
        let number = random_integer(min, max);

        assert!((min..=max).contains(&number))
    }
}
