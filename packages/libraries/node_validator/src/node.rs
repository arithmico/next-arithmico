use std::collections::HashSet;

use node::Node;
use translate_core::TranslatedMessage;

use crate::{Error, translations::translation_resolver};

/// A trait for validating properties of nodes.
pub trait NodeValidator {
    /// Validates that the node contains exactly one unknown symbol.
    fn validate_one_unknown_symbol(
        &self,
        known_symbols: &HashSet<&str>,
    ) -> Result<&Self, Error> {
        self.validate_unknown_symbol_count(known_symbols, 1)
    }

    /// Validates that the node contains exactly `expected` symbols which are
    /// not part of `known_symbols`.
    fn validate_unknown_symbol_count(
        &self,
        known_symbols: &HashSet<&str>,
        expected: usize,
    ) -> Result<&Self, Error>;

    fn validate_unknown_symbol_name(
        &self,
        known_symbols: &HashSet<&str>,
        unknown_symbol_name: &str,
    ) -> Result<&Self, Error>;
}

impl NodeValidator for Node {
    fn validate_unknown_symbol_count(
        &self,
        known_symbols: &HashSet<&str>,
        expected: usize,
    ) -> Result<&Self, Error> {
        let unknown_symbols = self
            .get_symbol_names()
            .into_iter()
            .filter(|symbol| !known_symbols.contains(*symbol))
            .collect::<HashSet<_>>();

        let actual = unknown_symbols.len();

        if actual < expected {
            let message = TranslatedMessage::new(
                "error.node.too_few_variables",
                translation_resolver,
            )
            .key("actual", actual)
            .key("expected", expected);

            return Err(Error::new(self, message));
        }

        if actual > expected {
            let message = TranslatedMessage::new(
                "error.node.too_many_variables",
                translation_resolver,
            )
            .key("actual", actual)
            .key("expected", expected);

            return Err(Error::new(self, message));
        }

        Ok(self)
    }

    fn validate_unknown_symbol_name(
        &self,
        known_symbols: &HashSet<&str>,
        unknown_symbol_name: &str,
    ) -> Result<&Self, Error> {
        let unknown_symbols = self
            .get_symbol_names()
            .into_iter()
            .filter(|symbol| !known_symbols.contains(*symbol))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        match &unknown_symbols[..] {
            [] => {
                let message = TranslatedMessage::new(
                    "error.node.too_few_variables",
                    translation_resolver,
                )
                .key("actual", 0)
                .key("expected", 1);

                return Err(Error::new(self, message));
            }
            [name] => {
                if *name != unknown_symbol_name {
                    let message = TranslatedMessage::new(
                        "error.node.invalid_variable_name",
                        translation_resolver,
                    )
                    .key("actual", name)
                    .key("expected", unknown_symbol_name);

                    return Err(Error::new(self, message));
                }
            }
            multiple => {
                let message = TranslatedMessage::new(
                    "error.node.too_many_variables",
                    translation_resolver,
                )
                .key("actual", multiple.len())
                .key("expected", 1);

                return Err(Error::new(self, message));
            }
        }

        Ok(self)
    }
}
