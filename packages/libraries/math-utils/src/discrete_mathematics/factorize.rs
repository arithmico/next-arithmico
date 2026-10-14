use std::collections::BTreeMap;

use num_prime::nt_funcs::factorize64;

pub fn factorize(n: u64) -> BTreeMap<u64, usize> {
    factorize64(n)
}

#[cfg(test)]
mod tests {
    use super::factorize;

    #[test]
    fn test_edge_cases() {
        assert_eq!(factorize(1).iter().collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_prime_numbers() {
        assert_eq!(factorize(2).iter().collect::<Vec<_>>(), vec![(&2, &1)]);
        assert_eq!(factorize(3).iter().collect::<Vec<_>>(), vec![(&3, &1)]);
        assert_eq!(factorize(13).iter().collect::<Vec<_>>(), vec![(&13, &1)]);
        assert_eq!(factorize(97).iter().collect::<Vec<_>>(), vec![(&97, &1)]);
    }

    #[test]
    fn test_perfect_powers() {
        assert_eq!(factorize(4).iter().collect::<Vec<_>>(), vec![(&2, &2)]); // 2^2
        assert_eq!(factorize(8).iter().collect::<Vec<_>>(), vec![(&2, &3)]); // 2^3
        assert_eq!(factorize(9).iter().collect::<Vec<_>>(), vec![(&3, &2)]); // 3^2
        assert_eq!(factorize(125).iter().collect::<Vec<_>>(), vec![(&5, &3)]); // 5^3
    }

    #[test]
    fn test_composite_numbers() {
        assert_eq!(
            factorize(6).iter().collect::<Vec<_>>(),
            vec![(&2, &1), (&3, &1)]
        ); // 2 * 3
        assert_eq!(
            factorize(12).iter().collect::<Vec<_>>(),
            vec![(&2, &2), (&3, &1)]
        ); // 2^2 * 3
        assert_eq!(
            factorize(28).iter().collect::<Vec<_>>(),
            vec![(&2, &2), (&7, &1)]
        ); // 2^2 * 7
        assert_eq!(
            factorize(100).iter().collect::<Vec<_>>(),
            vec![(&2, &2), (&5, &2)]
        ); // 2^2 * 5^2
        assert_eq!(
            factorize(2250).iter().collect::<Vec<_>>(),
            vec![(&2, &1), (&3, &2), (&5, &3)]
        ); // 2 * 3^2 * 5^3
    }

    #[test]
    fn test_large_prime() {
        assert_eq!(
            factorize(7919).iter().collect::<Vec<_>>(),
            vec![(&7919, &1)]
        );
    }
}
