#[cfg(test)]
mod tests {

    use crate::core::{
        context::Context, node::SerializeNode, parse::parse_statement,
    };

    fn compare(input: &str, expected: &str) {
        let context = Context::default();
        assert_eq!(
            parse_statement(input)
                .unwrap()
                .transform_before_serialization(&context)
                .serialize(&context),
            expected
        );
    }

    #[test]
    fn serialize_number_integer() {
        compare("2", "2");
    }

    #[test]
    fn serialize_number_float() {
        compare("2.3", "2.3");
    }

    #[test]
    fn serialize_number_to_scientific_notation() {
        compare("11234567", "1.12346 * 10^7");
    }

    #[test]
    fn serialize_symbol() {
        compare("abc", "abc");
    }

    #[test]
    fn serialize_boolean_true() {
        compare("true", "true");
    }

    #[test]
    fn serialize_boolean_false() {
        compare("false", "false");
    }

    #[test]
    fn serialize_negate() {
        compare("-2", "-2");
    }

    #[test]
    fn serialize_nested_negate() {
        compare("-(-2)", "-(-2)");
    }

    #[test]
    fn serialize_sum() {
        compare("1 + 2 + 3", "1 + 2 + 3");
    }

    #[test]
    fn serialize_sum_with_negate() {
        compare("1 - 2 + 3", "1 - 2 + 3");
    }

    #[test]
    fn serialize_product() {
        compare("1 * 2 * 3", "1 * 2 * 3");
    }

    #[test]
    fn serialize_division() {
        compare("2 / 3", "2 / 3");
    }

    #[test]
    fn serialize_product_with_division() {
        compare("1 * 2 / 3", "1 * 2 / 3");
    }

    #[test]
    fn serialize_power() {
        compare("2 ^ 3", "2^3");
    }

    #[test]
    fn serialize_power_with_sum_and_product() {
        compare("(1+2) ^ (3 * 4)", "(1 + 2)^(3 * 4)");
    }

    #[test]
    fn serialize_nested_power_1() {
        compare("1^(2^3)", "1^(2^3)");
    }

    #[test]
    fn serialize_nested_power_2() {
        compare("(1^2)^3", "(1^2)^3");
    }

    #[test]
    fn serialize_vector() {
        compare("[1,2,3]", "[1, 2, 3]");
    }

    #[test]
    fn serialize_empty_vector() {
        compare("[]", "[]");
    }

    #[test]
    fn serialize_nested_vector() {
        compare("[[1, 2], [3,4]]", "[[1, 2], [3, 4]]");
        compare("[[1, 2], [3, 4], [5, 6]]", "[[1, 2], [3, 4], [5, 6]]");
        compare(
            "[[[1], [2]], [[3], [4]], [[5], [6]]]",
            "[[[1], [2]], [[3], [4]], [[5], [6]]]",
        );
    }

    #[test]
    fn serialize_function_call_1() {
        compare("func()", "func()");
    }

    #[test]
    fn serialize_function_call_2() {
        compare("f(x,y)", "f(x, y)");
    }

    #[test]
    fn serialize_function_1() {
        compare("() -> 2", "() -> 2");
    }

    #[test]
    fn serialize_function_2() {
        compare("(x) -> x^2", "(x) -> x^2");
    }

    #[test]
    fn serialize_definition() {
        compare("a:=2", "a := 2");
    }

    #[test]
    fn serialize_and() {
        compare("a&b&c", "a & b & c");
    }

    #[test]
    fn serialize_or_1() {
        compare("a|b|c", "a | b | c");
    }

    #[test]
    fn serialize_or_and_1() {
        compare("a & b | c", "a & b | c");
    }

    #[test]
    fn serialize_or_and_2() {
        compare("a & (b | c)", "a & (b | c)");
    }

    #[test]
    fn serialize_or_and_3() {
        compare("(a & b) | c", "a & b | c");
    }
}
