#[macro_export]
macro_rules! function_executor_wrapper {
    ($args:ty, $implementation:path) => {{
        fn wrapper(
            arguments: &$crate::core::ArgumentMapping,
            context: &$crate::Context,
        ) -> Result<node::Node, $crate::core::EvaluateNodeError> {
            let typed =
                <$args as $crate::FromArgumentMapping>::from_argument_mapping(
                    arguments,
                )?;

            let result = $implementation(typed, context)?;

            Ok(result.into())
        }

        wrapper
    }};
}
