use nalgebra::{Dyn, MatrixViewMut, U1};

pub fn inverse(n: usize, matrix: &mut [f64]) -> Option<()> {
    let mut m = MatrixViewMut::from_slice_with_strides_generic(
        matrix,
        Dyn(n),
        Dyn(n),
        Dyn(n), // row stride: 1
        U1,     // column stride: n
    );

    m.try_inverse_mut().then_some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_inverse_1() {
        let mut matrix = [1.0, 2.0, 3.0, 4.0];
        let success = inverse(2, &mut matrix);

        assert!(success.is_some(), "matrix inversion was successful");
        assert_eq!(matrix, [-2.0, 1.0, 1.5, -0.5]);
    }

    #[test]
    fn test_matrix_inverse_2() {
        let mut matrix = [2.0, 1.0, 0.0, 1.0, 2.0, -2.0, 0.0, -1.0, 1.0];
        let success = inverse(3, &mut matrix);

        assert!(success.is_some(), "matrix inversion was successful");
        assert_eq!(matrix, [0.0, 1.0, 2.0, 1.0, -2.0, -4.0, 1.0, -2.0, -3.0]);
    }
}
