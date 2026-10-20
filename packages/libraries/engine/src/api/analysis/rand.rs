use common::Language;
use evaluator::{Error, FunctionArguments, FunctionEndpoint, Options};
use math_utils::random_float;
use node::Number;
use translate::RenderedTranslatedMessage;

pub struct RandArgs;

impl<'a> FunctionArguments<'a> for RandArgs {
    fn from_mapping(
        _argument_mapping: &'a evaluator::ArgumentMapping,
    ) -> Result<Self, Error> {
        Ok(Self)
    }

    fn signature() -> node::FunctionSignature {
        node::FunctionSignature::default()
    }

    fn function_description() -> translate::RenderedTranslatedMessage {
        let mut description = RenderedTranslatedMessage::default();

        description.add_message(
            Language::German,
            Ok("Gibt eine zufällige Zahl zwischen 0 (einschließlich) und 1 (ausschließlich) zurück."
                .to_string()),
        );
        description.add_message(
            Language::English,
            Ok("Returns a random number between 0 (inclusive) and 1 (exclusive)."
                .to_string()),
        );

        description
    }

    fn function_name() -> &'static str {
        "rand"
    }
}

pub struct RandEndpoint;

impl FunctionEndpoint for RandEndpoint {
    type Output = Number;
    type Arguments<'a> = RandArgs;

    // TODO: unit tests
    fn executor<'a>(
        RandArgs {}: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        Ok(Number::new(random_float()))
    }
}
