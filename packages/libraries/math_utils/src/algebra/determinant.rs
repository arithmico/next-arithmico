use nalgebra::{Dyn, MatrixView, U1};

pub fn matrix_determinant(n: usize, matrix: &[f64]) -> f64 {
    let m = MatrixView::from_slice_with_strides_generic(
        matrix,
        Dyn(n),
        Dyn(n),
        Dyn(n), // row stride: n
        U1,     // column stride: 1
    );

    m.determinant()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_determinant_1() {
        let mut matrix = [1.0];
        let result = matrix_determinant(1, &mut matrix);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_matrix_inverse_2() {
        let mut matrix = [1.0, 0.0, 0.0, 1.0];
        let result = matrix_determinant(2, &mut matrix);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_matrix_inverse_3() {
        let mut matrix = [1.0, 2.0, 3.0, 4.0];
        let result = matrix_determinant(2, &mut matrix);

        assert_eq!(result, -2.0);
    }
}
