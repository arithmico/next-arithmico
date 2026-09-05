use crate::greatest_common_divisor;

pub fn least_common_multiple(a: u64, b: u64) -> u64 {
    (a / greatest_common_divisor(a, b)) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcm_12_18_test() {
        assert_eq!(least_common_multiple(12, 18), 36);
    }

    #[test]
    fn lcm_6_7_test() {
        assert_eq!(least_common_multiple(6, 7), 42);
    }
}
