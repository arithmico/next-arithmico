use crate::core::{
    context::Context, node::SerializeNode, parse::parse_statement,
};

pub fn serialization_test(input: &str, expected: &str) {
    let context = Context::default();
    assert_eq!(
        parse_statement(input)
            .unwrap()
            .transform_before_serialization(&context)
            .serialize(&context),
        expected
    );
}
