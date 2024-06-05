use crate::{
    core::{
        context::Context,
        node::{Node, SerializeNode},
    },
    utils::serialize_utils::get_argument_separator,
};

use super::Function;

impl SerializeNode for Function {
    fn transform_before_serialization(&self, context: &Context) -> Node {
        Function::new(
            self.arguments.clone(),
            self.expression.transform_before_serialization(context),
        )
        .into()
    }

    fn serialize(&self, context: &Context) -> String {
        let arguments = self
            .arguments
            .join(&format!("{} ", get_argument_separator(context)));
        let expression = self.expression.serialize(context);
        format!("({}) -> {}", arguments, expression)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::context::Context,
        utils::test_utils::{
            serialization_test, serialization_test_with_context,
        },
        Language, Settings,
    };

    #[test]
    fn serialize_function_1() {
        serialization_test("() -> 2", "() -> 2");
    }

    #[test]
    fn serialize_function_2() {
        serialization_test("(x) -> x^2", "(x) -> x^2");
    }

    #[test]
    fn serialize_function_3() {
        serialization_test("(x, y) -> x + y", "(x, y) -> x + y");
    }

    #[test]
    fn serialize_function_german() {
        let context = Context {
            settings: Settings::new(5, Language::German),
            ..Context::default()
        };
        serialization_test_with_context(
            "(x, y) -> x + y",
            "(x; y) -> x + y",
            &context,
        );
    }
}
