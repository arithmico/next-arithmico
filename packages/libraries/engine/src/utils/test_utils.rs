use crate::core::{
    context::Context,
    node::{Node, SerializeNode},
};

pub fn serialization_test(input: &str, expected: &str) {
    let context = Context::default();
    assert_eq!(
        Node::parse(input)
            .unwrap()
            .transform_before_serialization(&context)
            .serialize(&context),
        expected
    );
}
