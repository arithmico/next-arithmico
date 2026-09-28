use approx::relative_eq;

pub trait F64Extension {
    fn relative_epsilon_tolerance(&self) -> f64;
    fn is_close_to(&self, other: f64) -> bool;
    fn is_close_to_zero(&self) -> bool;
    fn is_close_to_multiple_of(&self, base: f64) -> bool;
    fn is_close_to_shifted_multiple_of(&self, base: f64, shift: f64) -> bool;
    fn is_in_closed_interval(&self, lower: f64, upper: f64) -> bool;
    fn is_in_open_interval(&self, lower: f64, upper: f64) -> bool;
    fn is_integer(&self) -> bool;
}

impl F64Extension for f64 {
    /// Computes a relative tolerance scaled to the magnitude of the value.
    /// This ensures stable comparisons for both very small and large values.
    /// The tolerance is defined as:
    /// `max(|self|, 1.0) * f64::EPSILON`
    /// Using `max(|self|, 1.0)` guarantees that:
    /// - for values with |self| >= 1, the tolerance scales proportionally (relative error),
    /// - for values with |self| < 1, the tolerance does not shrink below `f64::EPSILON`,
    ///   avoiding overly strict comparisons near zero.
    ///
    /// `f64::EPSILON` is the difference between 1.0 and the next representable `f64` value,
    /// i.e. the machine precision for values around 1.0.
    fn relative_epsilon_tolerance(&self) -> f64 {
        self.abs().max(1.0) * f64::EPSILON
    }

    /// Checks whether `self` is approximately equal to `other`.
    /// Uses a relative epsilon comparison based on the magnitude of both values.
    /// This should be preferred over direct equality (`==`) for floating-point values.
    /// Returns `false` if either value is `NaN`, `f64::INFINITY` or `f64::NEG_INFINITY`.
    fn is_close_to(&self, other: f64) -> bool {
        if self.is_nan()
            || other.is_nan()
            || self.is_infinite()
            || other.is_infinite()
        {
            return false;
        }

        relative_eq!(
            *self,
            other,
            epsilon = self
                .relative_epsilon_tolerance()
                .max(other.relative_epsilon_tolerance())
        )
    }

    /// Checks whether `self` is approximately zero.
    /// This is equivalent to comparing `|self|` against a scaled epsilon tolerance.
    /// Useful for detecting numerical cancellation or rounding artifacts.
    fn is_close_to_zero(&self) -> bool {
        self.abs() <= self.relative_epsilon_tolerance()
    }

    /// Checks whether `self` is approximately a multiple of `base`.
    /// This uses modular arithmetic (`rem_euclid`) and checks whether the remainder
    /// is close to either `0` or `base`, accounting for floating-point drift.
    /// Returns `false` if `base` is approximately zero.
    ///
    /// Example:
    /// - `2π` is a multiple of `π`
    fn is_close_to_multiple_of(&self, base: f64) -> bool {
        if base.is_close_to_zero() {
            return false;
        }

        let r = self.rem_euclid(base);
        r.is_close_to_zero() || r.is_close_to(base)
    }

    /// Checks whether `self` is approximately equal to `shift + k * base` for some integer `k`.
    /// This is useful for detecting periodic offsets such as:
    /// - `π/2` relative to period `π`
    ///
    /// Returns `false` if `base` is approximately zero.
    fn is_close_to_shifted_multiple_of(&self, base: f64, shift: f64) -> bool {
        if base.is_close_to_zero() {
            return false;
        }

        let r = (self - shift).rem_euclid(base);
        r.is_close_to_zero() || r.is_close_to(base)
    }

    /// Checks whether `self` lies within the closed interval `[lower, upper]`,
    /// allowing for small floating-point inaccuracies at the boundaries.
    ///
    /// Boundary values are considered inside the interval if they are
    /// approximately equal to `lower` or `upper`.
    fn is_in_closed_interval(&self, lower: f64, upper: f64) -> bool {
        (lower < *self || lower.is_close_to(*self))
            && (*self < upper || upper.is_close_to(*self))
    }

    /// Checks whether `self` lies strictly within the open interval `(lower, upper)`.
    ///
    /// This performs a strict comparison and does not apply any tolerance
    /// at the boundaries.
    fn is_in_open_interval(&self, lower: f64, upper: f64) -> bool {
        lower < *self && *self < upper
    }

    /// Checks whether the value is approximately an integer.
    /// This method determines if the number is sufficiently close to its nearest
    /// integer (`self.round()`) using a tolerance-based comparison to account for
    /// floating-point rounding errors.
    /// Returns `false` for `NaN` and infinite values.
    ///
    /// # Rationale
    /// Due to floating-point precision limits, values that are mathematically
    /// integers may not be represented exactly (e.g. `2.0000000000000004`).
    /// This method provides a robust alternative to exact comparisons.
    fn is_integer(&self) -> bool {
        if self.is_nan() || self.is_infinite() {
            return false;
        }

        self.is_close_to(self.round())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn close_zero_detects_zero() {
        assert!(0.0.is_close_to_zero());
    }

    #[test]
    fn close_to_detects_equal_values() {
        assert!(1.0.is_close_to(1.0));
    }

    #[test]
    fn close_to_multiple_of_detects_pi_multiple() {
        assert!((2.0 * PI).is_close_to_multiple_of(PI));
    }

    #[test]
    fn close_to_multiple_of_shifted_detects_pi_over_two() {
        assert!((PI / 2.0).is_close_to_shifted_multiple_of(PI, PI / 2.0));
    }

    #[test]
    fn is_in_closed_interval() {
        assert!(2.0.is_in_closed_interval(1., 3.))
    }

    #[test]
    fn is_in_open_interval() {
        assert!(2.0.is_in_open_interval(1., 3.))
    }

    #[test]
    fn detects_integer() {
        assert!(2.0.is_integer());
    }

    #[test]
    fn detects_almost_integer() {
        assert!((2.0 + 1e-16).is_integer());
    }

    #[test]
    fn rejects_non_integer() {
        assert!(!2.1.is_integer());
    }
}
