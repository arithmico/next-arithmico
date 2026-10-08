/*
    Reference:
    Fred G. Gustavson, David W. Walker. Algorithms for in-place matrix transposition.
    Concurrency and Computation. Practice and Experience. 2019(31), e5071.
    <https://doi.org/10.1002/cpe.5071>
*/

/// Transposes a row-major matrix in place.
///
/// Based on the GCD Transpose algorithm described in Section 5.3,
/// in particular Algorithms 6-9 of the reference.
///
/// The algorithms in the reference operate on matrices stored in
/// column-major order (CMO). A `rows × columns` matrix stored in
/// row-major order (RMO) has the same linear representation as its
/// `columns × rows` transpose stored in CMO. Therefore, the dimensions
/// are reversed when entering the CMO algorithms.
///
/// Unlike Algorithms 6 and 8 in the reference, this implementation does
/// not use an out-of-place workspace. Empty remainder matrices (`r == 0`)
/// terminate the corresponding recursion directly.
///
/// # Reference
///
/// * F. G. Gustavson, D. W. Walker (2018), "Algorithms for in-place
///   matrix transposition", Concurrency and Computation, 2019 31, e5071.
///   <https://doi.org/10.1002/cpe.5071>
pub fn transpose(rows: usize, columns: usize, matrix: &mut [f64]) {
    if rows == 0 || columns == 0 {
        return;
    }

    if rows == columns {
        square_transpose(matrix, rows);
        return;
    }

    // row-major order (rows × columns) has the same linear representation as
    // column-major order (columns × rows).
    let n = columns;
    let m = rows;

    if n >= m {
        column_transpose(matrix, n, m);
    } else {
        row_transpose(matrix, n, m);
    }
}

/// Transposes an `n × n` square matrix in place.
fn square_transpose(matrix: &mut [f64], n: usize) -> usize {
    let mut nswaps = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            matrix.swap(i * n + j, j * n + i);
            nswaps += 1;
        }
    }

    nswaps
}

/// Based on the `exchange` operation described in Section 4.2.
///
/// Exchanges two contiguous subvectors:
///
/// ```text
/// A B -> B A
/// ```
///
/// where `A` has length `p` and `B` has length `q`.
fn exchange(
    matrix: &mut [f64],
    start: usize,
    mut p: usize,
    mut q: usize,
) -> usize {
    let mut nswaps = 0;
    let mut start = start;

    while p != 0 && q != 0 {
        if p >= q {
            // Exchange b_1 ... b_q with a_1 ... a_q.
            for i in 0..q {
                matrix.swap(start + i, start + p + i);
                nswaps += 1;
            }

            start += q;
            p -= q;
        } else {
            // Exchange a_1 ... a_p with b_(q-p+1) ... b_q.
            for i in 0..p {
                matrix.swap(start + i, start + q + i);
                nswaps += 1;
            }

            q -= p;
        }
    }

    nswaps
}

/// Unshuffles `m` pairs of contiguous vectors:
///
/// Each `a` vector has length `la` and each `b` vector has length `lb`.
///
/// Implements Algorithm 5 (`unshuffle`) from of the reference.
///
/// This is the divide-and-conquer (DAC) unshuffle algorithm described
/// in Section 5.1.
fn unshuffle(matrix: &mut [f64], la: usize, lb: usize, m: usize) -> usize {
    if m <= 1 {
        return 0;
    }

    if matrix.len() != (la + lb) * m {
        unreachable!();
    }

    // Algorithm 5:
    // m1 = largest power of 2 less than m
    let m1 = 1usize << (m - 1).ilog2();

    let (first, second) = matrix.split_at_mut((la + lb) * m1);

    let mut nswaps = 0;

    nswaps += unshuffle(first, la, lb, m1);
    nswaps += unshuffle(second, la, lb, m - m1);

    // After the recursive unshuffles:
    //
    // [a0 ... a(m1-1)]
    // [b0 ... b(m1-1)]
    // [a(m1) ... a(m-1)]
    // [b(m1) ... b(m-1)]
    //
    // Exchange the middle two blocks.
    nswaps += exchange(matrix, la * m1, lb * m1, la * (m - m1));

    nswaps
}

/// Shuffles `m` pairs of vectors in place:
///
/// Implements the inverse of the DAC `unshuffle` operation from
/// Section 5.1 / Algorithm 5 of Gustavson and Walker.
///
/// `shuffle` itself is defined by Equation (3) in Section 2.
fn shuffle(matrix: &mut [f64], la: usize, lb: usize, m: usize) -> usize {
    if m <= 1 {
        return 0;
    }

    if matrix.len() != (la + lb) * m {
        unreachable!();
    }

    // Again like Algorithm 5:
    // m1 = largest power of 2 less than m
    let m1 = 1usize << (m - 1).ilog2();

    // Initially:
    //
    // A_first | A_second | B_first | B_second
    //
    // Exchange A_second and B_first to obtain:
    //
    // A_first | B_first | A_second | B_second
    let mut nswaps = exchange(matrix, la * m1, la * (m - m1), lb * m1);

    // The two groups are now contiguous and can be shuffled
    // independently.
    let (first, second) = matrix.split_at_mut((la + lb) * m1);

    nswaps += shuffle(first, la, lb, m1);
    nswaps += shuffle(second, la, lb, m - m1);

    nswaps
}

/// Algorithm 6: `columnTranspose`.
fn column_transpose(matrix: &mut [f64], n: usize, m: usize) -> usize {
    let q = n / m;
    let r = n % m;

    let mut nswaps = unshuffle(matrix, q * m, r, m);

    let (a1, a2) = matrix.split_at_mut(q * m * m);

    nswaps += partition(a1, q, m);

    if r != 0 {
        nswaps += row_transpose(a2, r, m);
    }

    nswaps
}

/// Algorithm 7: `partition`.
fn partition(matrix: &mut [f64], q: usize, n: usize) -> usize {
    if q == 1 {
        return square_transpose(matrix, n);
    }

    let q2 = q / 2;
    let q1 = q - q2;

    let mut nswaps = unshuffle(matrix, q1 * n, q2 * n, n);

    let (a1, a2) = matrix.split_at_mut(q1 * n * n);

    nswaps += partition(a1, q1, n);
    nswaps += partition(a2, q2, n);

    nswaps
}

/// Algorithm 8: `rowTranspose`.
fn row_transpose(matrix: &mut [f64], n: usize, m: usize) -> usize {
    let q = m / n;
    let r = m % n;

    let (a1, a2) = matrix.split_at_mut(q * n * n);

    let mut nswaps = 0;

    if r != 0 {
        nswaps += column_transpose(a2, n, r);
    }

    nswaps += join(a1, q, n);
    nswaps += shuffle(matrix, q * n, r, n);

    nswaps
}

/// Algorithm 9: `join`.
fn join(matrix: &mut [f64], q: usize, n: usize) -> usize {
    if q == 1 {
        return square_transpose(matrix, n);
    }

    let q2 = q / 2;
    let q1 = q - q2;

    let (a1, a2) = matrix.split_at_mut(q1 * n * n);

    let mut nswaps = join(a1, q1, n);
    nswaps += join(a2, q2, n);
    nswaps += shuffle(matrix, q1 * n, q2 * n, n);

    nswaps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_transpose_2x2() {
        let mut matrix = [1.0, 2.0, 3.0, 4.0];
        transpose(2, 2, &mut matrix);

        assert_eq!(matrix, [1.0, 3.0, 2.0, 4.0]);
    }

    #[test]
    fn test_matrix_transpose_2x3_1() {
        let mut matrix = [1.0, 2.0, 4.0, 3.0, 3.0, 5.0];
        transpose(2, 3, &mut matrix);

        assert_eq!(matrix, [1.0, 3.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_matrix_transpose_2x3_2() {
        let mut matrix = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        transpose(2, 3, &mut matrix);

        assert_eq!(matrix, [1.0, 4.0, 2.0, 5.0, 3.0, 6.0,]);
    }

    #[test]
    fn test_matrix_transpose_5x2() {
        let mut matrix = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

        transpose(5, 2, &mut matrix);

        assert_eq!(
            matrix,
            [1.0, 3.0, 5.0, 7.0, 9.0, 2.0, 4.0, 6.0, 8.0, 10.0,]
        );
    }
}
