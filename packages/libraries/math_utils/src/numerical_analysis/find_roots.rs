use thiserror::Error;
use translate_core::{Translatable, TranslatedMessage};

use crate::{
    numerical_analysis::enclose_zero::{EnclosingZeroError, enclose_zero},
    translation_provider::translation_resolver,
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

impl Translatable for FindRootsError {
    fn translate(
        &self,
        language: translate_core::Language,
    ) -> Result<String, translate_core::TranslationError> {
        match self {
            FindRootsError::NonFiniteBoundary { name, value } => {
                TranslatedMessage::new(
                    "engine.api.error.generic_runtime_error.find_roots.non_finite_boundary",
                    translation_resolver
                )
                .key("name", name)
                .key("value", value)
            }
            FindRootsError::InvalidInterval { start, end } => {
                TranslatedMessage::new(
                    "engine.api.error.generic_runtime_error.find_roots.invalid_interval",
                    translation_resolver,
                )
                .key("start", start)
                .key("end", end)
            }
            FindRootsError::NonFiniteFunctionValue { x, value } => {
                TranslatedMessage::new(
                "engine.api.error.generic_runtime_error.find_roots.non_finite_function_value",
                    translation_resolver,
                )
                .key("x", x)
                .key("value", value)
            }
            FindRootsError::EncloseZero { a, b, .. } => {
                TranslatedMessage::new(
                "engine.api.error.generic_runtime_error.find_roots.enclose_zero",
                    translation_resolver,
                )
                .key("a", a)
                .key("b", b)
            }
        }.translate(language)
    }
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
    F: Fn(f64) -> Option<f64>,
{
    const STEP_SIZE: f64 = 0.125;

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
    let mut previous: Option<(f64, f64)> = None;

    let step_count = ((end - start) / STEP_SIZE).ceil() as usize;

    for index in 0..=step_count {
        let b = if index == step_count {
            end
        } else {
            start + index as f64 * STEP_SIZE
        };

        let Some(fb) = f(b) else {
            previous = None;
            continue;
        };

        if !fb.is_finite() {
            previous = None;
            continue;
        }

        // Exact sampled root.
        if fb == 0.0 {
            push_unique(&mut roots, b);
        }

        // Ordinary root enclosed by a sign change.
        if let Some((a, fa)) = previous {
            if fa != 0.0 && fb != 0.0 && fa.signum() != fb.signum() {
                let enclosed_function = |x: f64| f(x).unwrap_or(f64::NAN);

                let root = enclose_zero(&enclosed_function, a, b).map_err(
                    |source| FindRootsError::EncloseZero { a, b, source },
                )?;

                push_unique(&mut roots, root);
            }
        }

        // maybe: Detection of a local minimum of |f|. This could avoid errors for roots with an
        // even multiplicity, where the sign does not change.

        previous = Some((b, fb));
    }

    roots.sort_by(f64::total_cmp);

    Ok(roots)
}

fn push_unique(roots: &mut Vec<f64>, candidate: f64) {
    const DUPLICATE_TOLERANCE: f64 = 1.0e-14;

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
        let f = |x: f64| Some(x.sin());

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

            let result =
                if (half_turns - nearest_integer).abs() <= f64::EPSILON * 4.0 {
                    0.0
                } else {
                    degrees.to_radians().sin()
                };

            Some(result)
        };

        let roots = find_roots(&f, -180.0, -1.0)?;

        let normalized_roots: Vec<f64> =
            roots.into_iter().map(|root| root / 180.0).collect();

        assert_roots_close(normalized_roots, &[-1.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = 1 / x in [-10.0, 10.0]
    #[test]
    fn reciprocal_has_no_zero() -> Result<(), FindRootsError> {
        let f = |x: f64| {
            if x == 0.0 { None } else { Some(1.0 / x) }
        };

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert!(roots.is_empty(), "expected no roots for 1/x, got {roots:?}");

        Ok(())
    }

    /// f(x) = x^3 - 4 * x^2 + 3 in \[-2, 5\]
    #[test]
    fn finds_roots_of_first_cubic_polynomial() -> Result<(), FindRootsError> {
        let f = |x: f64| Some(x.powi(3) - 4.0 * x.powi(2) + 3.0);

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
        let f = |x: f64| Some(x.powi(34) - 1234.323_23);

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
        let f = |n: f64| Some((1.0 + n) * n / 2.0 - 34.0);

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
        let f = |x: f64| Some(x.powi(3) + x.powi(2) - 17.0 * x + 15.0);

        let roots = find_roots(&f, -6.0, 4.0)?;

        assert_roots_close(roots, &[-5.0, 1.0, 3.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = cbrt(x) in \[-2.0, 2.0\]
    #[test]
    fn finds_zero_of_cube_root_function() -> Result<(), FindRootsError> {
        let f = |x: f64| Some(x.cbrt());

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

            Some(
                500.0 * (x_25_over_4 - 1.0)
                    / (x_1_over_4 * (1.0 / x_25_over_4))
                    - 10_000.0,
            )
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

            Some(500.0 * geometric_sum / x_25_over_4 - 10_000.0)
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

            Some(x * annuity_factor * discount_factor - 250_000.0)
        };

        let roots = find_roots(&f, 10_000.0, 30_000.0)?;

        assert_roots_close(roots, &[1.932356194934123e4], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = sqrt(x) in \[-10, 10\]
    #[test]
    fn skips_values_outside_function_domain() -> Result<(), FindRootsError> {
        let f = |x: f64| {
            if x < 0.0 { None } else { Some(x.sqrt()) }
        };

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert_roots_close(roots, &[0.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = sqrt(x) in \[-10, 0\]
    #[test]
    fn find_root_sqrt_although_skipping_domain() -> Result<(), FindRootsError> {
        let f = |x: f64| {
            if x < 0.0 { None } else { Some(x.sqrt()) }
        };

        let roots = find_roots(&f, -10.0, 0.0)?;

        assert_roots_close(roots, &[0.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = (-11 + 18 * x)^2 + (2 - 6 * x)^2 - 25 in \[-10, 10\]
    #[test]
    fn find_root_two_squared_terms() -> Result<(), FindRootsError> {
        let f = |x: f64| {
            Some((-11.0 + 18.0 * x).powi(2) + (2.0 - 6.0 * x).powi(2) - 25.)
        };

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert_roots_close(
            roots,
            &[3.333333333333333e-1, 8.333333333333334e-1],
            ROOT_TOLERANCE,
        );

        Ok(())
    }

    /// f(x) = 100 - 420 * x + 360 * x^2 in \[-10, 10\]
    #[test]
    fn find_root_quadratic_function() -> Result<(), FindRootsError> {
        let f = |x: f64| Some(100.0 - 420.0 * x + 360.0 * x.powi(2));

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert_roots_close(
            roots,
            &[3.333333333333333e-1, 8.333333333333334e-1],
            ROOT_TOLERANCE,
        );

        Ok(())
    }

    /// f(x) = 100 * 0.87 ^ x - 1 in \[0, 50\]
    #[test]
    fn find_root_exponenttial_function() -> Result<(), FindRootsError> {
        let f = |x: f64| Some(100.0 * (0.87_f64).powf(x) - 1.0);

        let roots = find_roots(&f, 0.0, 50.0)?;

        assert_roots_close(roots, &[3.306837442646557e1], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = x^3 - 2*x^2 + x in \[-10, 10\]
    #[test]
    fn find_root_cubic_function() -> Result<(), FindRootsError> {
        let f = |x: f64| Some(x.powi(3) - 2.0 * x.powi(2) + x);

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert_roots_close(roots, &[0.0, 1.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = (x^2 - 3*x + 2) * (x - 2) in \[-10, 10\]
    #[test]
    fn find_root_cubic_function_2() -> Result<(), FindRootsError> {
        let f = |x: f64| Some((x.powi(2) - 3.0 * x + 2.0) * (x - 2.0));

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert_roots_close(roots, &[1.0, 2.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = x^3 - 18*x^2 + 81*x in \[-10, 10\]
    #[test]
    fn find_root_cubic_function_3() -> Result<(), FindRootsError> {
        let f = |x: f64| Some(x.powi(3) - 18.0_f64 * x.powi(2) + 81.0 * x);

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert_roots_close(roots, &[0.0, 9.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = x^2 - (9/5)*x + 4/5 in \[-10, 10\]
    #[test]
    fn find_root_quadratic_function_2() -> Result<(), FindRootsError> {
        let f = |x: f64| Some(x.powi(2) - (9.0 / 5.0) * x + 4.0 / 5.0);

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert_roots_close(roots, &[0.8, 1.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = (1/120) * x^2 - 2*x + 120 in \[0, 200\]
    #[test]
    fn find_root_quadratic_function_3() -> Result<(), FindRootsError> {
        let f = |x: f64| Some((1.0 / 120.0) * x.powi(2) - 2.0 * x + 120.0);

        let roots = find_roots(&f, 0.0, 200.0)?;

        assert_roots_close(roots, &[120.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = (200 + 10*x)^2 + (300 + 10*x)^2 + (5*x - 10)^2 - 1600^2 in \[-100_000, 1000\]
    #[test]
    fn find_root_quadratic_function_4() -> Result<(), FindRootsError> {
        let f = |x: f64| {
            Some(
                (200.0 + 10.0 * x).powi(2)
                    + (300.0 + 10.0 * x).powi(2)
                    + (5.0 * x - 10.0).powi(2)
                    - 1600.0_f64.powi(2),
            )
        };

        let roots = find_roots(&f, -100_000.0, 1000.0)?;

        assert_roots_close(
            roots,
            &[-128.224081806131, 84.2240818061307], // with Wolfram Alpha: https://www.wolframalpha.com/input?i=solve%28%28200+%2B+10*x%29%5E2+%2B+%28300+%2B+10*x%29%5E2+%2B+%285*x+-+10%29%5E2+-+1600%5E2%29
            ROOT_TOLERANCE,
        );

        Ok(())
    }

    /// f(x) = 3*x^2 - 6*x + 3 in \[-10, 10\]
    #[test]
    fn find_root_quadratic_function_5() -> Result<(), FindRootsError> {
        let f = |x: f64| Some(3.0 * x.powi(2) - 6.0 * x + 3.0);

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert_roots_close(roots, &[1.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = 100*x^16 - 15 in \[-10, 10\]
    #[test]
    fn find_root_degree_16_function() -> Result<(), FindRootsError> {
        let f = |x: f64| Some(100.0 * x.powi(16) - 15.0);

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert_roots_close(
            roots,
            &[-0.8881896410448874, 0.8881896410448868],
            ROOT_TOLERANCE,
        );

        Ok(())
    }

    /// f(x) = 1.5*e^x in \[-100, 100\]
    #[test]
    fn find_root_exponential_function_with_euler_base()
    -> Result<(), FindRootsError> {
        let f = |x: f64| Some(1.5_f64 * x.exp());

        let roots = find_roots(&f, -100.0, 100.0)?;

        assert_roots_close(roots, &[], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = 1.5*x^2 - x + x^3 in \[-10, 10\]
    #[test]
    fn find_root_cubic_function_4() -> Result<(), FindRootsError> {
        let f = |x: f64| Some(1.5_f64 * x.powi(2) - x + x.powi(3));

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert_roots_close(roots, &[-2.0, 0.0, 0.5], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = 7*x - 14*x^2 in \[-10, 10\]
    #[test]
    fn find_root_quadratic_function_6() -> Result<(), FindRootsError> {
        let f = |x: f64| Some(7.0 * x - 14.0 * x.powi(2));

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert_roots_close(roots, &[0.0, 0.5], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = 2*x*e^(1 - x^2) in \[-10, 10\]
    #[test]
    fn find_root_exponential_function_with_euler_base_2()
    -> Result<(), FindRootsError> {
        let f = |x: f64| Some(2.0 * x * (1.0 - x.powi(2)).exp());

        let roots = find_roots(&f, -10.0, 10.0)?;

        assert_roots_close(roots, &[0.0], ROOT_TOLERANCE);

        Ok(())
    }

    /// f(x) = x^2 - 10_000 in \[-10_000, 100_000\]
    #[test]
    fn find_root_quadratic_function_7() -> Result<(), FindRootsError> {
        let f = |x: f64| Some(x.powi(2) - 10_000.0);

        let roots = find_roots(&f, -10_000.0, 100_000.0)?;

        assert_roots_close(roots, &[-100.0, 100.0], ROOT_TOLERANCE);

        Ok(())
    }
}
