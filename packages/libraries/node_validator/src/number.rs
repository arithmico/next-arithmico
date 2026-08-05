use float_utils::F64Extension;
use node::Number;
use translate_core::TranslatedMessage;

use crate::{Error, translations::translation_resolver};

pub trait NumberValidator {
    fn validate_inside_closed_interval(
        &self,
        min: f64,
        max: f64,
    ) -> Result<&Self, Error>;

    fn validate_inside_open_interval(
        &self,
        min: f64,
        max: f64,
    ) -> Result<&Self, Error>;

    fn validate_integer(&self) -> Result<&Self, Error>;

    fn validate_less_than(&self, value: f64) -> Result<&Self, Error>;

    fn validate_less_than_or_equal(&self, value: f64) -> Result<&Self, Error>;

    fn validate_greater_than(&self, value: f64) -> Result<&Self, Error>;

    fn validate_greater_than_or_equal(
        &self,
        value: f64,
    ) -> Result<&Self, Error>;

    fn validate_equal(&self, value: f64) -> Result<&Self, Error>;

    fn validate_not_equal(&self, value: f64) -> Result<&Self, Error>;

    fn validate_positive(&self) -> Result<&Self, Error> {
        self.validate_greater_than_or_equal(0.0)
    }

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
