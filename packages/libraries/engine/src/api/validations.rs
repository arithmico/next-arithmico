use float_utils::F64Extension;
use node::Number;

use crate::core::EvaluateNodeError;

pub trait NumberValidation {
    fn validate_closed_interval(
        &self,
        min: f64,
        max: f64,
    ) -> Result<&Self, EvaluateNodeError>;
    fn validate_open_interval(
        &self,
        min: f64,
        max: f64,
    ) -> Result<&Self, EvaluateNodeError>;
    fn validate_non_negative(&self) -> Result<&Self, EvaluateNodeError>;
    fn validate_integer(&self) -> Result<&Self, EvaluateNodeError>;
    fn validate_non_zero(&self) -> Result<&Self, EvaluateNodeError>;
}

impl NumberValidation for Number {
    fn validate_closed_interval(
        &self,
        min: f64,
        max: f64,
    ) -> Result<&Self, EvaluateNodeError> {
        let value = self.value;

        if !value.is_in_closed_interval(min, max) {
            let err = EvaluateNodeError::invalid_parameter_value(
            "engine.api.error.invalid_parameter_value.closed_interval.out_of_bounds",
            )
            .key("lower", min)
            .key("upper", max)
            .build()
            .with_tracable(self);

            Err(err)
        } else {
            Ok(self)
        }
    }

    fn validate_open_interval(
        &self,
        min: f64,
        max: f64,
    ) -> Result<&Self, EvaluateNodeError> {
        let value = self.value;

        if !value.is_in_open_interval(min, max) {
            let err = EvaluateNodeError::invalid_parameter_value(
            "engine.api.error.invalid_parameter_value.open_interval.out_of_bounds",
            )
            .key("lower", min)
            .key("upper", max)
            .build()
            .with_tracable(self);

            Err(err)
        } else {
            Ok(self)
        }
    }

    fn validate_non_negative(&self) -> Result<&Self, EvaluateNodeError> {
        let value = self.value;

        if value.is_sign_negative() {
            let err = EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.invalid_parameter_value.negative_value",
            )
            .build()
            .with_tracable(self);

            Err(err)
        } else {
            Ok(self)
        }
    }

    fn validate_integer(&self) -> Result<&Self, EvaluateNodeError> {
        let value = self.value;

        if !value.is_integer() {
            let err = EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.invalid_parameter_value.non_integer",
            )
            .build()
            .with_tracable(self);

            Err(err)
        } else {
            Ok(self)
        }
    }

    fn validate_non_zero(&self) -> Result<&Self, EvaluateNodeError> {
        let value = self.value;

        if value.is_close_to_zero() {
            let err = EvaluateNodeError::invalid_parameter_value(
                "engine.api.error.invalid_parameter_value.zero",
            )
            .build()
            .with_tracable(self);

            Err(err)
        } else {
            Ok(self)
        }
    }
}
