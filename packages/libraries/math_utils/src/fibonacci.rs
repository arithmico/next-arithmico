pub fn fibonacci(n: u64) -> u64 {
    let mut a = 0u64;
    let mut b = 1u64;
    let mut bit = n.isolate_highest_one();
    while bit != 0 {
        let d = a * (2 * b - a);
        let e = a.pow(2) + b.pow(2);
        a = d;
        b = e;

        if (n & bit) != 0 {
            let c = a + b;
            a = b;
            b = c;
        }

        bit >>= 1;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_test() {
        let values = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
        let output = values.iter().enumerate().map(|(i, _)| fibonacci(i as u64)).collect::<Vec<_>>();
        assert_eq!(values, output);
    }
}