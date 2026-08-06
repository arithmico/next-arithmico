use thiserror::Error;

use crate::numerical_analysis::enclose_zero::{
    EnclosingZeroError, enclose_zero,
};

#[derive(Debug, Error)]
pub enum FindRootsError {
    #[error("The interval boundary `{name}` must be finite, got {value}")]
    NonFiniteBoundary { name: &'static str, value: f64 },
    #[error(
        "Invalid search interval: start ({start}) must be smaller than end ({end})"
    )]
    InvalidInterval { start: f64, end: f64 },
    #[error("The function returned a non-finite value at x = {x}: {value}")]
    NonFiniteFunctionValue { x: f64, value: f64 },
    #[error("zero enclosure failed in interval [{a}, {b}]: {source}")]
    EncloseZero {
        a: f64,
        b: f64,
        #[source]
        source: EnclosingZeroError,
    },
}

/// Finds all detectable zeros of `f` in the interval `[start, end]`.
///
/// The interval is scanned in fixed-size subintervals. Exact zeros at sampled
/// points and zeros enclosed by a sign change are refined with
/// [`enclose_zero`].
///
/// Returns the roots in ascending search order. Zeros without a sign change,
/// such as roots of even multiplicity, may not be detected.
///
/// # Errors
///
/// Returns [`FindRootsError`] if the interval is invalid, a boundary or
/// function value is not finite, or refinement of an enclosed zero fails.
pub fn find_roots<F>(
    f: &F,
    start: f64,
    end: f64,
) -> Result<Vec<f64>, FindRootsError>
where
    F: Fn(f64) -> f64,
{
    const STEP_SIZE: f64 = 0.25;

    if !start.is_finite() {
        return Err(FindRootsError::NonFiniteBoundary {
            name: "start",
            value: start,
        });
    }
    if !end.is_finite() {
        return Err(FindRootsError::NonFiniteBoundary {
            name: "end",
            value: end,
        });
    }

    if start >= end {
        return Err(FindRootsError::InvalidInterval { start, end });
    }

    let mut roots = Vec::new();
    let mut a = start;
    let mut fa = f(a);

    if !fa.is_finite() {
        return Err(FindRootsError::NonFiniteFunctionValue { x: a, value: fa });
    }

    if fa == 0.0 {
        push_unique(&mut roots, a);
    }

    while a < end {
        let b = (a + STEP_SIZE).min(end);
        let fb = f(b);

        if !fb.is_finite() {
            return Err(FindRootsError::NonFiniteFunctionValue {
                x: b,
                value: fb,
            });
        }

        if fb == 0.0 {
            push_unique(&mut roots, b);
        } else if fa.signum() != fb.signum() {
            let root = enclose_zero(f, a, b).map_err(|source| {
                FindRootsError::EncloseZero { a, b, source }
            })?;

            push_unique(&mut roots, root);
        }

        a = b;
        fa = fb;
    }

    Ok(roots)
}

fn push_unique(roots: &mut Vec<f64>, candidate: f64) {
    const DUPLICATE_TOLERANCE: f64 = 1.0e-10;

    let is_duplicate = roots
        .iter()
        .any(|root| (root - candidate).abs() <= DUPLICATE_TOLERANCE);

    if !is_duplicate {
        roots.push(candidate);
    }
}

#[cfg(test)]
mod tests {
    use super::{FindRootsError, find_roots};
    use std::f64::consts::PI;

    const ROOT_TOLERANCE: f64 = 1.0e-14;

    fn assert_roots_close(
        mut actual: Vec<f64>,
        expected: &[f64],
        tolerance: f64,
    ) {
        actual.sort_by(f64::total_cmp);

        assert_eq!(
            actual.len(),
            expected.len(),
            "different number of roots: \n
            actual:   {actual:?} \n
            expected: {expected:?}"
        );

        for (index, (actual_root, expected_root)) in
            actual.iter().zip(expected).enumerate()
        {
            let error = (actual_root - expected_root).abs();
            let allowed = tolerance.max(tolerance * expected_root.abs());

            assert!(
                error <= allowed,
                "root {index} differs: \
                 actual={actual_root:.15e}, \
                 expected={expected_root:.15e}, \
                 error={error:.5e}, \
                 allowed={allowed:.5e}"
            );
        }
    }

    /// f(x) = sin(x) / pi in \[-6 * pi, 6 * pi\]
    #[test]
    fn finds_zeros_of_sine_between_minus_six_and_six_pi()
    -> Result<(), FindRootsError> {
        let f = |x: f64| x.sin();

        let roots = find_roots(&f, -6.5 * PI, 6.5 * PI)?
            .into_iter()
            .map(|root| root / PI)
            .collect();

        assert_roots_close(
            roots,
            &[
                -6.0, -5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0,
                5.0, 6.0,
            ],
            ROOT_TOLERANCE,
        );

        Ok(())
    }

    /// f(x) = sin(x) in \[-180, -1\]
    #[test]
    fn finds_minus_180_degrees_as_sine_zero() -> Result<(), FindRootsError> {
        let f = |degrees: f64| {
            let half_turns = degrees / 180.0;
            let nearest_integer = half_turns.round();

            if (half_turns - nearest_integer).abs() <= f64::EPSILON * 4.0 {
                0.0
            } else {
                degrees.to_radians().sin()
            }
        };

        let roots = find_roots(&f, -180.0, -1.0)?;

        let normalized_roots: Vec<f64> =
            roots.into_iter().map(|root| root / 180.0).collect();

        assert_roots_close(normalized_roots, &[-1.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = 1 / x in \[-10.0, 10.0\]
    #[test]
    fn reciprocal_has_no_zero() -> Result<(), FindRootsError> {
        let f = |x: f64| 1.0 / x;

        let error = find_roots(&f, -10.0, 10.0)
            .expect_err("1/x is not finite at x = 0");

        if let FindRootsError::NonFiniteFunctionValue { x, value } = error {
            assert_eq!(x, 0.0);
            assert!(
                value.is_infinite(),
                "expected an infinite function value, got {value}"
            );
        } else {
            panic!("expected NonFiniteFunctionValue, got {error:?}");
        }

        Ok(())
    }

    /// f(x) = x^3 - 4 * x^2 + 3 in \[-2, 5\]
    #[test]
    fn finds_roots_of_first_cubic_polynomial() -> Result<(), FindRootsError> {
        let f = |x: f64| x.powi(3) - 4.0 * x.powi(2) + 3.0;

        let roots = find_roots(&f, -2.0, 5.0)?;

        assert_roots_close(
            roots,
            &[-7.912878474779200e-1, 1.0, 3.791287847477920e0],
            ROOT_TOLERANCE,
        );

        Ok(())
    }

    /// f(x) = x^34 - 1234.32323 in \[-2.0, 2.0\]
    #[test]
    fn finds_both_real_roots_of_even_power_equation()
    -> Result<(), FindRootsError> {
        let f = |x: f64| x.powi(34) - 1234.323_23;

        let roots = find_roots(&f, -2.0, 2.0)?;

        assert_roots_close(
            roots,
            &[-1.232890140415423e0, 1.232890140415423e0],
            ROOT_TOLERANCE,
        );

        Ok(())
    }

    /// f(x) = (1 + n) * n / 2 - 34 in \[-10.0, 10.0\]
    #[test]
    fn solves_triangular_number_equation() -> Result<(), FindRootsError> {
        let f = |n: f64| (1.0 + n) * n / 2.0 - 34.0;

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert_roots_close(
            roots,
            &[-8.761355820929154e0, 7.761355820929150e0],
            ROOT_TOLERANCE,
        );

        Ok(())
    }

    /// f(x) = x^3 + x^2 - 17 * x + 15 in \[-6.0, 4.0\]
    #[test]
    fn finds_integer_roots_of_second_cubic_polynomial()
    -> Result<(), FindRootsError> {
        let f = |x: f64| x.powi(3) + x.powi(2) - 17.0 * x + 15.0;

        let roots = find_roots(&f, -6.0, 4.0)?;

        assert_roots_close(roots, &[-5.0, 1.0, 3.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = cbrt(x) in \[-2.0, 2.0\]
    #[test]
    fn finds_zero_of_cube_root_function() -> Result<(), FindRootsError> {
        let f = |x: f64| x.cbrt();

        let roots = find_roots(&f, -2.0, 2.0)?;

        assert_roots_close(roots, &[0.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) =  500 * (x^(25/4)-1) / (x^(1/4) * 1/x^(25/4)) - 10000 in \[1.0, 2.0\]
    #[test]
    fn solves_first_fractional_power_equation() -> Result<(), FindRootsError> {
        let f = |x: f64| {
            let x_25_over_4 = x.powf(25.0 / 4.0);
            let x_1_over_4 = x.powf(1.0 / 4.0);

            500.0 * (x_25_over_4 - 1.0) / (x_1_over_4 * (1.0 / x_25_over_4))
                - 10_000.0
        };

        let roots = find_roots(&f, 1.0, 2.0)?;

        assert_roots_close(roots, &[1.299760973766279e0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = 500 * (x^(25/4)-1)/(x^(1/4)-1)*1/x^(25/4) - 10000 in \[1.0, 2.0\]
    #[test]
    fn solves_second_fractional_power_equation() -> Result<(), FindRootsError> {
        let f = |x: f64| {
            let y = x.powf(1.0 / 4.0);
            let x_25_over_4 = y.powi(25);

            let geometric_sum: f64 =
                (0..25).map(|exponent| y.powi(exponent)).sum();

            500.0 * geometric_sum / x_25_over_4 - 10_000.0
        };

        let roots = find_roots(&f, 1.0, 2.0)?;

        assert_roots_close(roots, &[1.073_784_518_627_854_5], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = x * (1.022^15 - 1)/(1.022-1) * 1/1.022^(15-1) - 250000 in \[10000, 30000\]
    #[test]
    fn solves_annuity_equation() -> Result<(), FindRootsError> {
        const RATE: f64 = 1.022;
        const PERIODS: i32 = 15;

        let f = |x: f64| {
            let annuity_factor = (RATE.powi(PERIODS) - 1.0) / (RATE - 1.0);

            let discount_factor = 1.0 / RATE.powi(PERIODS - 1);

            x * annuity_factor * discount_factor - 250_000.0
        };

        let roots = find_roots(&f, 10_000.0, 30_000.0)?;

        assert_roots_close(roots, &[1.932356194934123e4], ROOT_TOLERANCE);

        Ok(())
    }
}
