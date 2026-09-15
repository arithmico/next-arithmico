use nalgebra::{DMatrix, DMatrixView, DVectorView, Dyn, U1};
use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum EquationSolveError {
    #[error(
        "coefficient matrix and constant vector have incompatible dimensions"
    )]
    ShapeError,
    #[error("linear equation system has infinitely many solutions")]
    InfiniteSolutions,
    #[error("linear equation system has no solution")]
    NoSolution,
}

pub fn solve_linear_equation(
    n: usize,
    coefficients: &[f64],
    constants: &[f64],
) -> Result<Vec<f64>, EquationSolveError> {
    if coefficients.len() != n * n || constants.len() != n {
        return Err(EquationSolveError::ShapeError);
    }

    let matrix = DMatrixView::from_slice_with_strides_generic(
        coefficients,
        Dyn(n),
        Dyn(n),
        Dyn(n),
        U1,
    );
    let constants = DVectorView::from_slice(constants, n);

    // see decompositions:
    // https://www.baeldung.com/cs/solving-system-linear-equations
    if let Some(solution) = matrix.lu().solve(&constants) {
        return Ok(solution.as_slice().to_vec());
    }

    let rank = matrix.rank(1e-12);

    let augmented = DMatrix::from_fn(n, n + 1, |row, column| {
        if column < n {
            matrix[(row, column)]
        } else {
            constants[row]
        }
    });

    let augmented_rank = augmented.rank(1e-12);

    if rank < augmented_rank {
        Err(EquationSolveError::NoSolution)
    } else {
        Err(EquationSolveError::InfiniteSolutions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_solve_2x2_1() {
        //  2x + 3y = -6
        // -3x - 4y =  7
        //
        // x = 3
        // y = -4
        let matrix = [2.0, 3.0, -3.0, -4.0];
        let b = [-6.0, 7.0];

        let solution = solve_linear_equation(2, &matrix, &b).unwrap();

        assert_eq!(solution, [3.0, -4.0]);
    }

    #[test]
    fn test_matrix_solve_2x2_2() {
        // x     = 2
        // x + y = 3
        //
        // x = 2
        // y = 1
        let matrix = [1.0, 0.0, 1.0, 1.0];
        let b = [2.0, 3.0];

        let solution = solve_linear_equation(2, &matrix, &b).unwrap();

        assert_eq!(solution, [2.0, 1.0]);
    }

    #[test]
    fn test_matrix_solve_5x5() {
        // a                 = 1
        // a + b             = 2
        // a + b + c         = 3
        // a + b + c + d     = 4
        // a + b + c + d + e = 5
        //
        // a = b = c = d = e = 1
        let matrix = [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
            0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ];
        let b = [1.0, 2.0, 3.0, 4.0, 5.0];

        let solution = solve_linear_equation(5, &matrix, &b).unwrap();

        assert_eq!(solution, [1.0, 1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_no_solution() {
        // x + y = 3
        // 2x + 2y = 4
        let matrix = [1.0, 1.0, 2.0, 2.0];
        let b = [3.0, 4.0];

        let result = solve_linear_equation(2, &matrix, &b);

        assert_eq!(result, Err(EquationSolveError::NoSolution));
    }

    #[test]
    fn test_infinite_solutions() {
        // -6x + 4y =  2
        //  3x - 2y = -1
        let matrix = [-6.0, 4.0, 3.0, -2.0];
        let b = [2.0, -1.0];

        let result = solve_linear_equation(2, &matrix, &b);

        assert_eq!(result, Err(EquationSolveError::InfiniteSolutions));
    }
}
