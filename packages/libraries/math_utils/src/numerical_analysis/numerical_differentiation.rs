/// Numerically approximates a derivative of `f` at `x0`.
///
/// The derivative is approximated using a centered finite-difference stencil.
/// Finite-difference weights are generated with the Fornberg algorithm and are
/// evaluated repeatedly with decreasing step sizes.
///
/// The step size starts at `0.5 * max(|x0|, 1)` and is divided by two after
/// each iteration. The difference between two consecutive derivative
/// approximations is used as an error estimate. Iteration stops when either
/// the requested absolute/relative tolerance is reached or the estimated error
/// increases significantly, indicating that floating-point roundoff is
/// beginning to dominate.
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
/// The best derivative approximation found during adaptive step-size
/// refinement.
///
/// # Numerical Method
///
/// The implementation combines:
///
/// * centered finite-difference stencils,
/// * Fornberg's recursive algorithm for finite-difference weights,
/// * symmetry correction of centered stencil weights,
/// * Neumaier compensated summation, and
/// * adaptive step-size refinement.
///
/// # References
///
/// * B. Fornberg (1998), "Calculation of Weights in Finite Difference
///   Formulas", SIAM Review 40, pp. 685-691.
///   <https://doi.org/10.1137/S0036144596322507>
/// * SciPy's adaptive numerical differentiation implementation, which uses
///   repeated step-size reduction, differences between consecutive estimates
///   as an error estimate (constants values are overtook), and termination
///   when roundoff error begins to dominate:
///   <https://github.com/scipy/scipy/blob/main/scipy/differentiate/_differentiate.py>
pub fn calculate_numerical_derivative<F>(
    f: &F,
    x0: f64,
    order: usize,
) -> f64
where
    F: Fn(f64) -> f64,
{
    const FORMULA_ORDER: usize = 6;
    const MAX_ITERATIONS: usize = 10;
    const STEP_FACTOR: f64 = 2.0;
    const ABSOLUTE_TOLERANCE: f64 = 1e-14;
    const RELATIVE_TOLERANCE: f64 = 1e-14;

    let scale = x0.abs().max(1.0);
    let mut adaptive_step_size = 0.5 * scale; // = h

    let stencil_offsets = generate_stencil_offsets(order, FORMULA_ORDER);

    let mut finite_difference_weights = weights(0.0, &stencil_offsets, order);
    enforce_weight_symmetry(&mut finite_difference_weights, order);

    let mut previous = calculate_finite_difference_with_step_size(
        f,
        x0,
        order,
        adaptive_step_size,
        &stencil_offsets,
        &finite_difference_weights,
    );

    let mut best = previous;
    let mut best_error = f64::INFINITY;
    let mut previous_error = f64::INFINITY;

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

        let error = (current - previous).abs();

        if error < best_error {
            best = current;
            best_error = error;
        }

        let tolerance = ABSOLUTE_TOLERANCE + RELATIVE_TOLERANCE * current.abs();

        if error <= tolerance {
            return current;
        }

        if error > previous_error * 10.0 {
            break;
        }

        previous = current;
        previous_error = error;
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
    f: &F,
    x0: f64,
    order: usize,
    h: f64,
    stencil_offsets: &[f64],
    finite_difference_weights: &[f64],
) -> f64
where
    F: Fn(f64) -> f64,
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
fn generate_stencil_offsets(order: usize, formula_order: usize) -> Vec<f64> {
    // Mathematical determination of the stencil half-width based on order and accuracy.
    // For a central scheme, the minimal number of points is forced to be odd to ensure symmetry.
    let half_stencil = (2 * ((order + 1) / 2) - 1 + formula_order) / 2;
    // alternative: (order + fromula_order - 1) / 2;

    let half_range = half_stencil as i32;

    (-half_range..=half_range)
        .map(|offset| offset as f64)
        .collect()
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
/// SIAM Review 40, pp. 685-691. <https://doi.org/10.1137/S0036144596322507>
/// * Stabilization fix inspired by the Julia scientific machine learning ecosystem (SciML/DiffEqOperators.jl):
///   <https://github.com/SciML/DiffEqOperators.jl/blob/master/src/derivative_operators/fornberg.jl>
fn weights(z: f64, x: &[f64], m: usize) -> Vec<f64> {
    let n = x.len();
    let mut c1 = 1.0;
    let mut c4 = x[0] - z;

    let stride = m + 1; // Number of columns (0 to m - order of derivatives)
    let mut c = vec![0.0_f64; n * stride]; // result matrix as flatted vec

    c[0 * stride + 0] = 1.0;

    for i in 1..n {
        let mn = i.min(m);
        let mut c2 = 1.0;
        let c5 = c4;
        c4 = x[i] - z;

        for j in 0..i {
            let c3 = x[i] - x[j];
            c2 = c2 * c3;

            if j == i - 1 {
                for k in (1..=mn).rev() {
                    let k_float = k as f64;
                    c[i * stride + k] = c1
                        * (k_float * c[(i - 1) * stride + (k - 1)]
                            - c5 * c[(i - 1) * stride + k])
                        / c2;
                }

                c[i * stride + 0] = -c1 * c5 * c[(i - 1) * stride + 0] / c2
            }

            for k in (1..=mn).rev() {
                let k_float = k as f64;
                c[j * stride + k] = (c4 * c[j * stride + k]
                    - k_float * c[j * stride + (k - 1)])
                    / c3;
            }

            c[j * stride + 0] = c4 * c[j * stride + 0] / c3
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

    const EPSILON: f64 = 1e-14;

    /// f(x) = x^2, f'(2) = 4
    #[test]
    fn test_x_squared_first_derivative_default() {
        let f = |x: f64| x.powi(2);
        let result = calculate_numerical_derivative(&f, 2.0, 1);
        assert!((result - 4.0).abs() < EPSILON);
    }

    /// f(x) = x^2, f''(3) = 2
    #[test]
    fn test_x_squared_second_derivative() {
        let f = |x: f64| x.powi(2);
        let result = calculate_numerical_derivative(&f, 3.0, 2);
        assert!((result - 2.0).abs() < EPSILON);
    }

    /// f(x) = x^2, f''(pi^2) = 2
    #[test]
    fn test_x_squared_second_derivative_at_pi_squared() {
        let f = |x: f64| x.powi(2);
        let x0 = PI.powi(2);
        let result = calculate_numerical_derivative(&f, x0, 2);
        assert!((result - 2.0).abs() < EPSILON);
    }

    /// f(x) = x, f'(0) = 1
    #[test]
    fn test_linear_first_derivative() {
        let f = |x: f64| x;
        let result = calculate_numerical_derivative(&f, 0.0, 1);
        assert!((result - 1.0).abs() < EPSILON);
    }

    /// f(x) = x^2, f'''(2) = 0
    #[test]
    fn test_x_squared_third_derivative_is_zero() {
        let f = |x: f64| x.powi(2);
        let result = calculate_numerical_derivative(&f, 2.0, 3);
        assert!((result - 0.0).abs() < EPSILON);
    }

    /// f(x) = x^3, f''''(4) = 0
    #[test]
    fn test_x_cubed_fourth_derivative_is_zero() {
        let f = |x: f64| x.powi(3);
        let result = calculate_numerical_derivative(&f, 4.0, 4);
        assert!((result - 0.0).abs() < EPSILON);
    }

    /// f(x) = x^7, f^{'8}(3) = 0
    #[test]
    fn test_x_seventh_eighth_derivative_is_zero() {
        let f = |x: f64| x.powi(7);
        let result = calculate_numerical_derivative(&f, 3.0, 8);
        assert!((result - 0.0).abs() < 1e0);
    }

    /// f(x) = (x + 1) / (x - 2 * x^4), f'(3/5) = 0
    #[test]
    fn test_complex_rational_function() {
        let f = |x: f64| (x + 1.0) / (x - 2.0 * x.powi(4));
        let x0 = 3.0 / 5.0;
        let result = calculate_numerical_derivative(&f, x0, 1);
        assert!((result - 12.9631466419802077).abs() < 1e-10);
    }

    /// f(x) = x^20, f'(3) = 2.324523 * 10^10
    #[test]
    fn test_high_power_first_derivative() {
        let f = |x: f64| x.powi(20);
        let result = calculate_numerical_derivative(&f, 3.0, 1);
        let expected = 20.0 * 3.0_f64.powi(19);

        assert!((result - expected).abs() < 1e-2);
    }
}
