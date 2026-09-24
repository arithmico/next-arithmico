use std::usize;

pub fn transpose(rows: usize, columns: usize, matrix: &mut [f64]) {
    let mut buffer = vec![0.0; matrix.len()];

    for i in 0..rows {
        for j in 0..columns {
            buffer[j * rows + i] = matrix[i * columns + j];
        }
    }

    matrix.copy_from_slice(&buffer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_transpose_1() {
        let mut matrix = [1.0, 2.0, 3.0, 4.0];
        transpose(2, 2, &mut matrix);

        assert_eq!(matrix, [1.0, 3.0, 2.0, 4.0]);
    }

    #[test]
    fn test_matrix_transpose_2() {
        let mut matrix = [1.0, 2.0, 4.0, 3.0, 3.0, 5.0];
        transpose(2, 3, &mut matrix);

        assert_eq!(matrix, [1.0, 3.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_matrix_transpose_3() {
        let mut matrix = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        transpose(2, 3, &mut matrix);

        assert_eq!(matrix, [1.0, 4.0, 2.0, 5.0, 3.0, 6.0,]);
    }
}
