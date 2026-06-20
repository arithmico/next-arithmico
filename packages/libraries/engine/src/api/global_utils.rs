#[allow(dead_code)]
pub trait F64Extension {
    fn relative_epsilon_tolerance(&self) -> f64;
    fn is_close_to(&self, other: f64) -> bool;
    fn is_close_to_zero(&self) -> bool;
    fn is_close_to_multiple_of(&self, base: f64) -> bool;
    fn is_close_to_shifted_multiple_of(&self, base: f64, shift: f64) -> bool;
    fn is_in_closed_interval(&self, lower: f64, upper: f64) -> bool;
    fn is_in_open_interval(&self, lower: f64, upper: f64) -> bool;
}

impl F64Extension for f64 {
    fn relative_epsilon_tolerance(&self) -> f64 {
        self.abs().max(1.0) * f64::EPSILON
    }

    fn is_close_to(&self, other: f64) -> bool {
        if self.is_nan() || other.is_nan() {
            return false;
        }

        if self == &other {
            return true;
        }
        let eps = self
            .relative_epsilon_tolerance()
            .max(other.relative_epsilon_tolerance());

        (self - other).abs() <= eps
    }

    fn is_close_to_zero(&self) -> bool {
        self.abs() <= self.relative_epsilon_tolerance()
    }

    fn is_close_to_multiple_of(&self, base: f64) -> bool {
        if base.is_close_to_zero() {
            return false;
        }

        let r = self.rem_euclid(base);
        r.is_close_to_zero() || r.is_close_to(base)
    }

    fn is_close_to_shifted_multiple_of(&self, base: f64, shift: f64) -> bool {
        if base.is_close_to_zero() {
            return false;
        }

        let r = (self - shift).rem_euclid(base);
        r.is_close_to_zero() || r.is_close_to(base)
    }

    fn is_in_closed_interval(&self, lower: f64, upper: f64) -> bool {
        (lower < *self || lower.is_close_to(*self))
            && (*self < upper || upper.is_close_to(*self))
    }

    fn is_in_open_interval(&self, lower: f64, upper: f64) -> bool {
        lower < *self && *self < upper
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
}
