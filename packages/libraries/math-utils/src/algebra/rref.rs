use nalgebra::{Dyn, MatrixViewMut, U1};

/// Returns reduced row echolon formed matrix with Gauss-Jordan Algorithm.
///
/// Inspired by:
/// source: <https://github.com/gnu-octave/octave/blob/default/scripts/linear-algebra/rref.m>
pub fn reduced_row_echelon_form(
    rows: usize,
    columns: usize,
    matrix: &mut [f64],
) -> Option<()> {
    const TOL: f64 = 1e-14;

    if matrix.len() != rows * columns {
        return None;
    }

    let mut matrix = MatrixViewMut::from_slice_with_strides_generic(
        matrix,
        Dyn(rows),
        Dyn(columns),
        Dyn(columns),
        U1,
    );

    let mut pivot_row = 0;

    for col in 0..columns {
        if pivot_row >= rows {
            break;
        }

        // find max row for pivot to increase numerical stability
        let mut max_row = pivot_row;
        for row in (pivot_row + 1)..rows {
            if matrix[(row, col)].abs() > matrix[(max_row, col)].abs() {
                max_row = row;
            }
        }

        // if value below epsilon, skip row
        if matrix[(max_row, col)].abs() < TOL {
            continue;
        }

        // swap rows if necassary
        if max_row != pivot_row {
            matrix.swap_rows(pivot_row, max_row);
        }

        // normalize leading entry to 1
        let pivot_val = matrix[(pivot_row, col)];
        matrix.set_row(pivot_row, &(matrix.row(pivot_row) / pivot_val));

        // eliminate all other entries in this column above and below pivot row element
        for row in 0..rows {
            if row != pivot_row {
                let factor = matrix[(row, col)];
                if factor.abs() > TOL {
                    let new_row =
                        matrix.row(row) - matrix.row(pivot_row) * factor;
                    matrix.set_row(row, &new_row);
                }
            }
        }

        pivot_row += 1;
    }

    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_rref_1() {
        let mut matrix = [1.0, 2.0, 3.0, 4.0];
        let success = reduced_row_echelon_form(2, 2, &mut matrix);

        assert!(
            success.is_some(),
            "Gauss-Jordan transformation was successful"
        );
        assert_eq!(matrix, [1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_matrix_rref_2() {
        let mut matrix = [2.0, 1.0, 0.0, 1.0, 2.0, -2.0, 0.0, -1.0, 1.0];
        let success = reduced_row_echelon_form(3, 3, &mut matrix);

        assert!(
            success.is_some(),
            "Gauss-Jordan transformation was successful"
        );
        assert_eq!(matrix, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_matrix_rref_rectangular_1() {
        let mut matrix = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let success = reduced_row_echelon_form(2, 3, &mut matrix);

        assert!(
            success.is_some(),
            "Gauss-Jordan transformation was successful"
        );
        assert_eq!(matrix, [1.0, 0.0, -1.0, 0.0, 1.0, 2.0]);
    }

    #[test]
    fn test_matrix_rref_rectangular_2() {
        let mut matrix = [1., 2., 0., 4., 0., 1., -1., 0., 1., 3., -2., 5.];
        let success = reduced_row_echelon_form(3, 4, &mut matrix);

        assert!(
            success.is_some(),
            "Gauss-Jordan transformation was successful"
        );
        assert_eq!(matrix, [1., 0., 0., 6., 0., 1., 0., -1., 0., 0., 1., -1.]);
    }

    #[test]
    fn test_matrix_rref_rectangular_3() {
        let mut matrix = [1., -2., 3., 2., 2., -3., 6., 3., -1., 5., -3., 1.];
        let success = reduced_row_echelon_form(4, 3, &mut matrix);

        assert!(
            success.is_some(),
            "Gauss-Jordan transformation was successful"
        );
        assert_eq!(matrix, [1., 0., 0., 0., 1., 0., 0., 0., 1., 0., 0., 0.]);
    }

    #[test]
    fn test_matrix_rref_dependent_rows() {
        let mut matrix = [1.0, 2.0, 2.0, 4.0];

        let success = reduced_row_echelon_form(2, 2, &mut matrix);

        assert!(
            success.is_some(),
            "Gauss-Jordan transformation was successful"
        );
        assert_eq!(matrix, [1.0, 2.0, 0.0, 0.0]);
    }
}
