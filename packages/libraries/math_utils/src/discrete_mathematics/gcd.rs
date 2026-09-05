pub fn greatest_common_divisor(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    }

    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_34_30_test() {
        assert_eq!(greatest_common_divisor(34, 30), 2);
    }

    #[test]
    fn gcd_4_12_test() {
        assert_eq!(greatest_common_divisor(4, 12), 4);
    }

    #[test]
    fn gcd_12_4_test() {
        assert_eq!(greatest_common_divisor(12, 4), 4);
    }

    #[test]
    fn gcd_63_22_test() {
        assert_eq!(greatest_common_divisor(63, 22), 1);
    }
}
