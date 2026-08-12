
use node::Function;
use translate_core::TranslatedMessage;

use crate::{Error, translations::translation_resolver};

/// A trait for validating properties of functions.
pub trait FunctionValidator {
    /// Validates that the function has exactly one argument.
    fn validate_one_argument(&self) -> Result<&Self, Error> {
        self.validate_argument_count(1)
    }

    /// Validates that the function has exactly the expected number of
    /// arguments.
    ///
    /// # Arguments
    ///
    /// * `expected` - The expected number of function arguments.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the number of arguments in the function
    /// signature differs from the expected number.
    fn validate_argument_count(&self, expected: usize) -> Result<&Self, Error>;

    /// Validates that the single function argument has the same name as the
    /// given unknown symbol.
    ///
    /// This method assumes that the function has exactly one argument.
    ///
    /// # Arguments
    ///
    /// * `unknown_symbol` - The unknown symbol found in the function body.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the function argument name differs from the
    /// unknown symbol.
    fn validate_argument_matches_unknown_symbol(
        &self,
        unknown_symbol: &str,
    ) -> Result<&Self, Error>;
}

impl FunctionValidator for Function {
    fn validate_argument_count(&self, expected: usize) -> Result<&Self, Error> {
        let actual = self.signature.arguments().len();

        if actual < expected {
            let message = TranslatedMessage::new(
                "error.function.too_few_arguments",
                translation_resolver,
            )
            .key("actual", actual)
            .key("expected", expected);

            return Err(Error::new(self, message));
        }

        if actual > expected {
            let message = TranslatedMessage::new(
                "error.function.too_many_arguments",
                translation_resolver,
            )
            .key("actual", actual)
            .key("expected", expected);

            return Err(Error::new(self, message));
        }

        Ok(self)
    }

    fn validate_argument_matches_unknown_symbol(
        &self,
        unknown_symbol: &str,
    ) -> Result<&Self, Error> {
        let arguments = self.signature.arguments();

        let argument = arguments.first().ok_or_else(|| {
            let message = TranslatedMessage::new(
                "error.function.too_few_arguments",
                translation_resolver,
            )
            .key("actual", arguments.len())
            .key("expected", 1);

            Error::new(self, message)
        })?;

        let argument_name = argument.get_name();

        if argument_name != unknown_symbol {
            let message = TranslatedMessage::new(
                "error.function.argument_symbol_mismatch",
                translation_resolver,
            )
            .key("argument", argument_name)
            .key("symbol", unknown_symbol);

            return Err(Error::new(self, message));
        }

        Ok(self)
    }
}
