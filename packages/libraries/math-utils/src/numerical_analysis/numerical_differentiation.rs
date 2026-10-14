/// Numerically approximates a derivative of `f` at `x0`.
///
/// The derivative is approximated using centered finite differences with
/// adaptively decreasing step sizes. Finite-difference weights are generated
/// once using Fornberg's recursive algorithm and reused for all step sizes.
///
/// Starting from `0.5 * max(|x0|, 1)`, the step size is divided by
/// `STEP_FACTOR` after each iteration. The resulting sequence of
/// finite-difference approximations
///
/// `D(h), D(h/r), D(h/r^2), ...`
///
/// is refined with multiple Richardson extrapolation levels. Each level
/// eliminates another term of the assumed truncation-error expansion. For the
/// centered finite-difference formulas used here, successive error exponents
/// differ by two:
///
/// `h^p, h^(p+2), h^(p+4), ...`.
///
/// Only the most recent Richardson row is retained:
///
/// `D(h), R1(h), R2(h), ...`.
///
/// When a new finite-difference approximation is available, this row is
/// updated in place. Consequently, multiple Richardson extrapolation requires
/// only `O(k)` storage for `k` extrapolation levels rather than storing the
/// complete extrapolation table.
///
/// The difference between the highest Richardson level and the level directly
/// below it is used as an error estimate. Iteration stops when this estimate
/// satisfies the absolute/relative tolerance or increases substantially,
/// indicating that floating-point roundoff and cancellation are beginning to
/// dominate.
///
/// # Parameters
///
/// * `f` - Function whose derivative is approximated.
/// * `x0` - Point at which the derivative is evaluated.
/// * `order` - Derivative order. For example, `1` requests the first
///   derivative and `2` the second derivative.
///
/// # Returns
///
/// The most accurate extrapolated derivative approximation encountered during
/// adaptive step-size refinement.
///
/// # Numerical Method
///
/// The implementation combines:
///
/// * centered finite-difference stencils,
/// * Fornberg's recursive algorithm for finite-difference weights,
/// * exact symmetry enforcement of centered stencil weights,
/// * Neumaier compensated summation,
/// * adaptive geometric step-size refinement, and
/// * multi-level Richardson extrapolation with bounded extrapolation depth.
///
/// # References
///
/// * B. Fornberg (1998), "Calculation of Weights in Finite Difference
///   Formulas", SIAM Review 40, pp. 685-691.
///   <https://doi.org/10.1137/S0036144596322507>
/// * Numdifftools combines finite-difference step sequences with multiple
///   Richardson extrapolation terms and models the error exponents as
///   `k_i = order + step * i`:
///   <https://github.com/pbrod/numdifftools>
/// * SciPy's numerical differentiation implementation uses adaptive
///   step-size refinement and detects the point at which roundoff error
///   begins to dominate. Its implementation also discusses Richardson
///   extrapolation as a numerical improvement:
///   <https://github.com/scipy/scipy/blob/main/scipy/differentiate/_differentiate.py>
pub fn calculate_numerical_derivative<F>(
    f: &mut F,
    x0: f64,
    order: usize,
) -> f64
where
    F: FnMut(f64) -> f64,
{
    const ERROR_ORDER: usize = 2;
    const MAX_ITERATIONS: usize = 10;
    const STEP_FACTOR: f64 = 2.0;
    const ABSOLUTE_TOLERANCE: f64 = f64::EPSILON;
    const RELATIVE_TOLERANCE: f64 = f64::EPSILON;

    const ERROR_ORDER_STEP: usize = 2;
    const MAX_RICHARDSON_LEVELS: usize = 3;

    let scale = x0.abs().max(1.0);
    let mut adaptive_step_size = 0.5 * scale; // = h

    let stencil_offsets = generate_stencil_offsets(order, ERROR_ORDER);

    let mut finite_difference_weights = weights(0.0, &stencil_offsets, order);
    enforce_weight_symmetry(&mut finite_difference_weights, order);

    let initial_approximation = calculate_finite_difference_with_step_size(
        f,
        x0,
        order,
        adaptive_step_size,
        &stencil_offsets,
        &finite_difference_weights,
    );

    let mut richardson_row = vec![initial_approximation];

    let mut best = initial_approximation;
    let mut best_error = f64::INFINITY;
    let mut error_last = f64::INFINITY;

    for _ in 0..MAX_ITERATIONS {
        adaptive_step_size /= STEP_FACTOR;

        let current = calculate_finite_difference_with_step_size(
            f,
            x0,
            order,
            adaptive_step_size,
            &stencil_offsets,
            &finite_difference_weights,
        );

        update_richardson_terms(
            &mut richardson_row,
            current,
            STEP_FACTOR,
            ERROR_ORDER,
            ERROR_ORDER_STEP,
            MAX_RICHARDSON_LEVELS,
        );

        let extrapolated = richardson_row
            .last()
            .copied()
            .unwrap_or(initial_approximation);

        let error = if richardson_row.len() >= MAX_RICHARDSON_LEVELS - 1 {
            let last = richardson_row.len() - 1;

            (richardson_row[last] - richardson_row[last - 1]).abs()
        } else {
            f64::INFINITY
        };

        if error < best_error {
            best = extrapolated;
            best_error = error;
        }

        let tolerance =
            ABSOLUTE_TOLERANCE + RELATIVE_TOLERANCE * extrapolated.abs();

        if error <= tolerance {
            return extrapolated;
        }

        // from: <https://github.com/scipy/scipy/blob/main/scipy/differentiate/_differentiate.py>
        if error > error_last * 10.0 {
            break;
        }

        error_last = error;
    }

    best
}

/// Evaluates a finite-difference derivative approximation for one fixed step
/// size.
///
/// The supplied stencil coordinates are dimensionless offsets around zero.
/// Each offset `s` is mapped to the actual function argument
/// `x0 + s * step_size`. The weighted function values are then divided by
/// `step_size.powi(order)` to obtain an approximation of the requested
/// derivative.
///
/// Using dimensionless stencil coordinates separates the geometry of the
/// finite-difference stencil from its physical step size and allows the same
/// weights to be reused during adaptive step-size refinement.
///
/// The weighted values are accumulated with [`neumaier_sum`] to reduce
/// floating-point cancellation error.
///
/// # Parameters
///
/// * `f` - Function to evaluate.
/// * `x0` - Point at which the derivative is approximated.
/// * `order` - Derivative order.
/// * `step_size` - Current finite-difference step size.
/// * `stencil_offsets` - Dimensionless stencil offsets around zero.
/// * `finite_difference_weights` - Weights corresponding to the stencil
///   offsets.
///
/// # References
///
/// The separation of relative stencil coordinates and step-size scaling is
/// consistent with finite-difference coefficient formulations based on
/// relative coordinates, including the SciML Fornberg implementation.
/// <https://github.com/SciML/DiffEqOperators.jl/blob/master/src/derivative_operators/fornberg.jl>
fn calculate_finite_difference_with_step_size<F>(
    f: &mut F,
    x0: f64,
    order: usize,
    h: f64,
    stencil_offsets: &[f64],
    finite_difference_weights: &[f64],
) -> f64
where
    F: FnMut(f64) -> f64,
{
    let weights = finite_difference_weights
        .iter()
        .zip(stencil_offsets.iter())
        .map(|(&weight, &offset)| weight * f(x0 + offset * h))
        .collect::<Vec<_>>();

    neumaier_sum(&weights) / h.powi(order as i32)
}

/// Generates symmetric integer offsets for a centered finite-difference
/// stencil.
///
/// The stencil is centered at zero and contains an odd number of points so
/// that every negative offset has a corresponding positive offset and the
/// evaluation point itself is included.
///
/// The stencil half-width is derived from the requested derivative order and
/// target accuracy. For example, a resulting half-width of `3` produces
///
/// `[-3, -2, -1, 0, 1, 2, 3]`.
///
/// # Parameters
///
/// * `order` - Derivative order.
/// * `accuracy` - Requested asymptotic accuracy order.
///
/// # Returns
///
/// Dimensionless stencil offsets centered around zero.
///
/// # References
///
/// The construction is inspired by Findiff's generation of centered
/// finite-difference offsets from derivative and accuracy orders:
/// <https://github.com/maroba/findiff/blob/master/findiff/coefs.py>
fn generate_stencil_offsets(order: usize, error_order: usize) -> Vec<f64> {
    // Mathematical determination of the stencil half-width based on order and accuracy.
    // For a central scheme, the minimal number of points is forced to be odd to ensure symmetry.
    let half_stencil = (2 * order.div_ceil(2) - 1 + error_order) / 2;
    // alternative: (order + error_order - 1) / 2;

    let half_range = half_stencil as i32;

    (-half_range..=half_range)
        .map(|offset| offset as f64)
        .collect()
}

/// approximation while retaining only the most recent extrapolation row.
///
/// Each element in `previous_row` represents one Richardson level for the
/// previous step size:
///
/// `D(h), R1(h), R2(h), ...`
///
/// See Wikipedia: <https://en.wikipedia.org/wiki/Richardson_extrapolation>.
///
/// Given a new finite-difference approximation `D(h / r)`, a new row is
/// generated recursively:
///
/// `D(h / r), R1(h / r), R2(h / r), ...`
///
/// The truncation-error exponents are assumed to follow
///
/// `p_k = initial_error_order + k * error_order_step`.
///
/// For centered finite differences, `error_order_step` is typically `2`
/// because the truncation-error expansion contains successive even powers
/// of the step size.
///
/// # Parameters
///
/// * `previous_row` - Richardson values from the previous step size.
/// * `approximation` - New finite-difference approximation at the smaller
///   step size.
/// * `step_factor` - Ratio between successive step sizes.
/// * `initial_error_order` - Leading truncation-error exponent.
/// * `error_order_step` - Difference between successive error exponents.
/// * `max_levels` - Maximum number of Richardson extrapolation levels.
///
/// # References
///
/// Numdifftools models Richardson error exponents as
/// `k_i = order + step * i` and removes a configurable number of terms from
/// the truncation-error expansion.
/// <https://github.com/pbrod/numdifftools>
/// * SciPy identifies one-step Richardson extrapolation of consecutive
///   derivative estimates as a possible improvement to its numerical
///   differentiation implementation:
///   <https://github.com/scipy/scipy/blob/main/scipy/differentiate/_differentiate.py>
// Extends a Richardson extrapolation sequence by one finite-difference
fn update_richardson_terms(
    row: &mut Vec<f64>,
    approximation: f64,
    step_factor: f64,
    initial_error_order: usize,
    error_order_step: usize,
    max_levels: usize,
) {
    let previous_len = row.len();
    let levels = previous_len.min(max_levels);

    // Replace D(h) with D(h / r), but retain the old D(h), which is needed
    // as the coarse value for the first Richardson level.
    let mut coarse = std::mem::replace(&mut row[0], approximation);

    for level in 1..=levels {
        let error_order = initial_error_order + (level - 1) * error_order_step;

        let factor = step_factor.powi(error_order as i32);

        // The preceding element has already been updated and therefore
        // represents the finer approximation at the current level.
        let fine = row[level - 1];

        let extrapolated = fine + (fine - coarse) / (factor - 1.0);

        if level < previous_len {
            // Replace the old value with the new extrapolation and retain
            // the old value as the coarse input for the next level.
            coarse = std::mem::replace(&mut row[level], extrapolated);
        } else {
            // The Richardson hierarchy grows by one level until max_levels
            // is reached.
            row.push(extrapolated);
        }
    }
}

/// Calculates optimal finite difference weights using the Fornberg algorithm.
///
/// This implementation is based on Bengt Fornberg's recursive algorithm for arbitrary,
/// non-uniformly spaced grid structures and arbitrary derivative orders. It includes a
/// post-processing numerical stabilization step to correct floating-point rounding errors.
///
/// # Parameters
///
/// * `z` - The evaluation point (target location) where approximations are to be accurate.
/// * `x` - A slice containing the coordinates of the grid points (the differential stencil).
/// * `m` - The highest derivative order for which weights are sought.
///
/// # Return Value
///
/// Returns one finite-difference weight for each point in `x`. This contains the computed and
/// stabilized coefficients for the highest requested derivative order `m`.
///
/// # Panics
///
/// The function will panic if:
/// * The slice `x` is empty.
///
/// # References
///
/// * B. Fornberg (1998): "Calculation of Weights in Finite Difference Formulas".
///   SIAM Review 40, pp. 685-691. <https://doi.org/10.1137/S0036144596322507>
/// * Stabilization fix inspired by the Julia scientific machine learning ecosystem (SciML/DiffEqOperators.jl):
///   <https://github.com/SciML/DiffEqOperators.jl/blob/master/src/derivative_operators/fornberg.jl>
fn weights(z: f64, x: &[f64], m: usize) -> Vec<f64> {
    let n = x.len();
    let mut c1 = 1.0;
    let mut c4 = x[0] - z;

    let stride = m + 1; // Number of columns (0 to m - order of derivatives)
    let mut c = vec![0.0_f64; n * stride]; // result matrix as flatted vec

    c[0] = 1.0;

    for i in 1..n {
        let mn = i.min(m);
        let mut c2 = 1.0;
        let c5 = c4;
        c4 = x[i] - z;

        for j in 0..i {
            let c3 = x[i] - x[j];
            c2 *= c3;

            if j == i - 1 {
                for k in (1..=mn).rev() {
                    let k_float = k as f64;
                    c[i * stride + k] = c1
                        * (k_float * c[(i - 1) * stride + (k - 1)]
                            - c5 * c[(i - 1) * stride + k])
                        / c2;
                }

                c[i * stride] = -c1 * c5 * c[(i - 1) * stride] / c2
            }

            for k in (1..=mn).rev() {
                let k_float = k as f64;
                c[j * stride + k] = (c4 * c[j * stride + k]
                    - k_float * c[j * stride + (k - 1)])
                    / c3;
            }

            c[j * stride] = c4 * c[j * stride] / c3
        }

        c1 = c2;
    }

    let mut last_col = (0..n).map(|i| c[i * stride + m]).collect::<Vec<_>>();

    // suggestion from: https://github.com/SciML/DiffEqOperators.jl/blob/master/src/derivative_operators/fornberg.jl
    // original issue: https://scicomp.stackexchange.com/questions/11249/numerical-derivative-and-finite-difference-coefficients-any-update-of-the-fornb
    //
    // This addresses the numerical instability issue that arises
    // when the sum of the stencil coefficients is not exactly 0 due to rounding errors.
    if m != 0 {
        // Calculates the sum of weight of the highest order
        let sum_c = last_col.iter().sum::<f64>();

        last_col[n / 2] -= sum_c;
    }

    last_col
}

/// Enforces the exact symmetry properties of centered finite-difference
/// weights.
///
/// For a stencil symmetric about zero, derivative weights have parity that
/// follows the derivative order:
///
/// * odd derivative orders are antisymmetric:
///   `w(-x) = -w(x)`, with zero weight at the center;
/// * even derivative orders are symmetric:
///   `w(-x) = w(x)`.
///
/// Floating-point computation of the weights can slightly violate these
/// identities. This function averages mirrored coefficient pairs to restore
/// the expected symmetry.
///
/// # Parameters
///
/// * `weights` - Finite-difference weights ordered according to a symmetric
///   stencil.
/// * `order` - Derivative order.
///
/// # Notes
///
/// SciPy explicitly enforces antisymmetry of centered first-derivative
/// weights after computing them in floating-point arithmetic. This function
/// generalizes the same idea to arbitrary derivative orders by applying the
/// corresponding odd/even parity relation.
///
/// # References
///
/// * SciPy numerical differentiation implementation:
///   <https://github.com/scipy/scipy/blob/main/scipy/differentiate/_differentiate.py>
fn enforce_weight_symmetry(weights: &mut [f64], order: usize) {
    let n = weights.len();
    let center = n / 2;

    // Odd derivative order: w(-x) = -w(x), with zero center weight.
    if order % 2 == 1 {
        weights[center] = 0.0;

        for i in 0..center {
            let j = n - 1 - i;

            let value = (weights[j] - weights[i]) / 2.0;

            weights[i] = -value;
            weights[j] = value;
        }
    } else {
        // Even derivative order: w(-x) = w(x).
        for i in 0..center {
            let j = n - 1 - i;

            let value = (weights[i] + weights[j]) / 2.0;

            weights[i] = value;
            weights[j] = value;
        }
    }
}

/// Computes a floating-point sum using Neumaier compensated summation.
///
/// Neumaier's algorithm is a refinement of compensated summation that keeps a
/// separate correction term for low-order bits lost during floating-point
/// addition. Unlike the basic Kahan formulation, it also handles the case in
/// which the next summand has a larger magnitude than the current partial sum.
///
/// This is useful for finite-difference formulas because positive and negative
/// weighted function values can be large while their final sum is small,
/// making naive summation susceptible to cancellation error.
///
/// # Parameters
///
/// * `values` - Floating-point values to sum.
///
/// # Returns
///
/// A compensated approximation of the sum of all values.
///
/// # References
///
/// * N. J. Higham, "The Accuracy of Floating Point Summation", SIAM Journal
///   on Scientific Computing.
fn neumaier_sum(values: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut correction = 0.0;

    for value in values {
        let next = sum + value;

        if sum.abs() >= value.abs() {
            correction += (sum - next) + value;
        } else {
            correction += (value - next) + sum;
        }

        sum = next;
    }

    sum + correction
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    const EPSILON: f64 = f64::EPSILON;

    /// f(x) = x^2, f'(2) = 4
    #[test]
    fn test_x_squared_first_derivative_default() {
        let mut f = |x: f64| x.powi(2);
        let result = calculate_numerical_derivative(&mut f, 2.0, 1);
        assert!((result - 4.0).abs() < EPSILON);
    }

    /// f(x) = x^2, f''(3) = 2
    #[test]
    fn test_x_squared_second_derivative() {
        let mut f = |x: f64| x.powi(2);
        let result = calculate_numerical_derivative(&mut f, 3.0, 2);
        assert!((result - 2.0).abs() < EPSILON);
    }

    /// f(x) = x^2, f''(pi^2) = 2
    #[test]
    fn test_x_squared_second_derivative_at_pi_squared() {
        let mut f = |x: f64| x.powi(2);
        let x0 = PI.powi(2);
        let result = calculate_numerical_derivative(&mut f, x0, 2);
        assert!((result - 2.0).abs() < EPSILON);
    }

    /// f(x) = x, f'(0) = 1
    #[test]
    fn test_linear_first_derivative() {
        let mut f = |x: f64| x;
        let result = calculate_numerical_derivative(&mut f, 0.0, 1);
        assert!((result - 1.0).abs() < EPSILON);
    }

    /// f(x) = x^2, f'''(2) = 0
    #[test]
    fn test_x_squared_third_derivative_is_zero() {
        let mut f = |x: f64| x.powi(2);
        let result = calculate_numerical_derivative(&mut f, 2.0, 3);
        assert!((result - 0.0).abs() < EPSILON);
    }

    /// f(x) = x^3, f''''(4) = 0
    #[test]
    fn test_x_cubed_fourth_derivative_is_zero() {
        let mut f = |x: f64| x.powi(3);
        let result = calculate_numerical_derivative(&mut f, 4.0, 4);
        assert!((result - 0.0).abs() < EPSILON);
    }

    /// f(x) = x^7, f^{'8}(3) = 0
    #[test]
    fn test_x_seventh_eighth_derivative_is_zero() {
        let mut f = |x: f64| x.powi(7);
        let result = calculate_numerical_derivative(&mut f, 3.0, 8);
        println!("{}", result);
        assert!((result - 0.0).abs() < EPSILON);
    }

    /// f(x) = x^8, f^{'8}(3) = 40320
    #[test]
    fn eighth_derivative_of_x_eighth() {
        let mut f = |x: f64| x.powi(8);
        let result = calculate_numerical_derivative(&mut f, 3.0, 8);
        assert!((result - 40320.0).abs() < EPSILON);
    }

    #[test]
    fn eighth_derivative_of_x_ninth() {
        let mut f = |x: f64| x.powi(9);
        let result = calculate_numerical_derivative(&mut f, 3.0, 8);
        let expected = 362_880.0 * 3.0; // 9! * 3 = 1_088_640
        assert!((result - expected).abs() < EPSILON);
    }

    /// f(x) = (x + 1) / (x - 2 * x^4), f'(3/5) = 0
    #[test]
    fn test_complex_rational_function() {
        let mut f = |x: f64| (x + 1.0) / (x - 2.0 * x.powi(4));
        let x0 = 3.0 / 5.0;
        let result = calculate_numerical_derivative(&mut f, x0, 1);
        assert!((result - 12.9631466419802077).abs() < 1e-12);
    }

    /// f(x) = x^20, f'(3) = 2.324523 * 10^10
    #[test]
    fn test_high_power_first_derivative() {
        let mut f = |x: f64| x.powi(20);
        let result = calculate_numerical_derivative(&mut f, 3.0, 1);
        let expected = 20.0 * 3.0_f64.powi(19); // 2.32452293400000000e10
        assert!((result - expected).abs() < 1e-3);
    }
}
