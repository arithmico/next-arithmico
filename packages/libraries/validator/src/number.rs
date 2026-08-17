use float_utils::F64Extension;
use node::Number;
use translate_core::TranslatedMessage;

use crate::{Error, translations::translation_resolver};

/// A trait for validating properties of numerical values.
///
/// All methods return `Result<&Self, Error>` to allow for fluent method
/// chaining when performing multiple validation checks.
pub trait NumberValidator {
    /// Validates that the number falls within a closed interval `[min, max]`.
    ///
    /// This means the value must be greater than or equal to `min`, and less
    /// than or equal to `max`.
    ///
    /// # Arguments
    ///
    /// * `min` - The inclusive lower bound.
    /// * `max` - The inclusive upper bound.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the value is strictly less than `min` or strictly
    /// greater than `max`.
    fn validate_inside_closed_interval(
        &self,
        min: f64,
        max: f64,
    ) -> Result<&Self, Error>;

    /// Validates that the number falls within an open interval `(min, max)`.
    ///
    /// This means the value must be strictly greater than `min`, and strictly
    /// less than `max`.
    ///
    /// # Arguments
    ///
    /// * `min` - The exclusive lower bound.
    /// * `max` - The exclusive upper bound.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the value is less than or equal to `min`, or
    /// greater than or equal to `max`.
    fn validate_inside_open_interval(
        &self,
        min: f64,
        max: f64,
    ) -> Result<&Self, Error>;

    /// Validates that the number is an integer.
    ///
    /// This ensures the value has no fractional component.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the value contains a non-zero fractional part.
    fn validate_integer(&self) -> Result<&Self, Error>;

    /// Validates that the number is strictly less than the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The exclusive upper limit to compare against.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if `self` is greater than or equal to `value`.
    fn validate_less_than(&self, value: f64) -> Result<&Self, Error>;

    /// Validates that the number is less than or equal to the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The inclusive upper limit to compare against.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if `self` is strictly greater than `value`.
    fn validate_less_than_or_equal(&self, value: f64) -> Result<&Self, Error>;

    /// Validates that the number is strictly greater than the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The exclusive lower limit to compare against.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if `self` is less than or equal to `value`.
    fn validate_greater_than(&self, value: f64) -> Result<&Self, Error>;

    /// Validates that the number is greater than or equal to the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The inclusive lower limit to compare against.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if `self` is strictly less than `value`.
    fn validate_greater_than_or_equal(
        &self,
        value: f64,
    ) -> Result<&Self, Error>;

    /// Validates that the number is equal to the specified value.
    ///
    /// This comparison safely accounts for floating-point rounding issues,
    /// meaning values that are practically identical despite minor precision
    /// loss are treated as equal.
    ///
    /// # Arguments
    ///
    /// * `value` - The target value to match.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if `self` does not equal `value` (falling outside
    /// the allowed floating-point tolerance).
    fn validate_equal(&self, value: f64) -> Result<&Self, Error>;

    /// Validates that the number is not equal to the specified value.
    ///
    /// This comparison safely accounts for floating-point rounding issues,
    /// meaning values must be distinctly different beyond standard
    /// floating-point inaccuracies to pass validation.
    ///
    /// # Arguments
    ///
    /// * `value` - The target value that must be avoided.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if `self` is equal to `value` (falling within the
    /// allowed floating-point tolerance).
    fn validate_not_equal(&self, value: f64) -> Result<&Self, Error>;

    /// Validates that the number is positive (or zero).
    ///
    /// By default, this uses `validate_greater_than_or_equal(0.0)`, meaning
    /// zero is considered a valid positive value in this context.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the value is strictly less than `0.0`.
    fn validate_positive(&self) -> Result<&Self, Error> {
        self.validate_greater_than_or_equal(0.0)
    }

    /// Validates that the number is strictly negative.
    ///
    /// By default, this uses `validate_less_than(0.0)`, meaning the value
    /// must be less than zero.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the value is greater than or equal to `0.0`.
    fn validate_negative(&self) -> Result<&Self, Error> {
        self.validate_less_than(0.0)
    }
}

impl NumberValidator for Number {
    fn validate_inside_closed_interval(
        &self,
        min: f64,
        max: f64,
    ) -> Result<&Self, Error> {
        if !self.value.is_in_closed_interval(min, max) {
            let message = TranslatedMessage::new(
                "error.number.closed_interval",
                translation_resolver,
            )
            .key("min", min)
            .key("max", max);
            return Err(Error::new(self, message));
        }
        Ok(self)
    }

    fn validate_inside_open_interval(
        &self,
        min: f64,
        max: f64,
    ) -> Result<&Self, Error> {
        if !self.value.is_in_open_interval(min, max) {
            let message = TranslatedMessage::new(
                "error.number.open_interval",
                translation_resolver,
            )
            .key("min", min)
            .key("max", max);
            return Err(Error::new(self, message));
        }
        Ok(self)
    }

    fn validate_integer(&self) -> Result<&Self, Error> {
        if !self.value.is_integer() {
            let message = TranslatedMessage::new(
                "error.number.integer",
                translation_resolver,
            )
            .key("value", self.value);
            return Err(Error::new(self, message));
        }
        Ok(self)
    }

    fn validate_less_than(&self, value: f64) -> Result<&Self, Error> {
        if self.value >= value {
            let message = TranslatedMessage::new(
                "error.number.less_than",
                translation_resolver,
            )
            .key("value", self.value)
            .key("v", value);
            return Err(Error::new(self, message));
        }
        Ok(self)
    }

    fn validate_less_than_or_equal(&self, value: f64) -> Result<&Self, Error> {
        if self.value > value {
            let message = TranslatedMessage::new(
                "error.number.less_than_or_equal",
                translation_resolver,
            )
            .key("value", self.value)
            .key("v", value);
            return Err(Error::new(self, message));
        }
        Ok(self)
    }

    fn validate_greater_than(&self, value: f64) -> Result<&Self, Error> {
        if self.value <= value {
            let message = TranslatedMessage::new(
                "error.number.greater_than",
                translation_resolver,
            )
            .key("value", self.value)
            .key("v", value);
            return Err(Error::new(self, message));
        }
        Ok(self)
    }

    fn validate_greater_than_or_equal(
        &self,
        value: f64,
    ) -> Result<&Self, Error> {
        if self.value < value {
            let message = TranslatedMessage::new(
                "error.number.greater_than_or_equal",
                translation_resolver,
            )
            .key("value", self.value)
            .key("v", value);
            return Err(Error::new(self, message));
        }
        Ok(self)
    }

    fn validate_equal(&self, value: f64) -> Result<&Self, Error> {
        if !self.value.is_close_to(value) {
            let message = TranslatedMessage::new(
                "error.number.equal",
                translation_resolver,
            )
            .key("value", self.value)
            .key("v", value);
            return Err(Error::new(self, message));
        }
        Ok(self)
    }

    fn validate_not_equal(&self, value: f64) -> Result<&Self, Error> {
        if self.value.is_close_to(value) {
            let message = TranslatedMessage::new(
                "error.number.not_equal",
                translation_resolver,
            )
            .key("v", value);
            return Err(Error::new(self, message));
        }
        Ok(self)
    }
}
