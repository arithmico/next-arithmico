use node::{Negate, Node, Number, Power, Product};

use crate::core::{
    Context, Serialize, SerializeNodeError, SerializeUtils,
    get_decimal_separator,
};

impl SerializeUtils for Number {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        if self.value == 0.0 {
            return Ok(Number::new_node(self.value));
        }

        let decimal_places = i32::from(&context.decimal_places);

        let magnitude = self.value.abs().log10().round() as i64;
        let magnitude_abs = if magnitude < 0 { -magnitude } else { magnitude };
        if magnitude_abs <= decimal_places as i64 {
            return if self.value < 0.0 {
                Ok(Negate::new(Number::new_node(self.value.abs())))
            } else {
                Ok(Number::new_node(self.value))
            };
        }

        let sign = self.value.signum();
        let factor = self.value.abs() * 10_f64.powi(-magnitude as i32);
        let scientific_notation = Product::new(vec![
            Number::new_node(factor).into(),
            Power::new(
                Number::new_node(10.0),
                if magnitude < 0 {
                    Node::from(Negate::new(Number::new_node(
                        magnitude.abs() as f64
                    )))
                } else {
                    Node::from(Number::new_node(magnitude.abs() as f64))
                },
            )
            .into(),
        ]);

        if sign > 0.0 {
            Ok(scientific_notation)
        } else {
            Ok(Negate::new(scientific_notation))
        }
    }

    fn child_requires_parenthesis(
        &self,
        _child: &Node,
        _position: usize,
    ) -> bool {
        false
    }
}

impl Serialize for Number {
    fn serialize(
        &self,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        if self.value == 0.0 {
            return Ok("0".into());
        }
        if self.value < 0.0 {
            return Err(SerializeNodeError::InvalidNode);
        }

        let decimal_places = usize::from(&context.decimal_places);
        let serialized_value = format!("{:.1$}", self.value, decimal_places);

        Ok(String::from(
            serialized_value
                .trim_end_matches("0")
                .trim_end_matches(".")
                .replace(".", &get_decimal_separator(context)),
        ))
    }
}

#[cfg(test)]
mod tests {

    use translate::Language;

    use crate::core::{DecimalPlaces, HostApi, Stack, serialize_node};

    use super::*;

    #[test]
    fn serialize_number_int() {
        assert_eq!(
            serialize_node(&Number::new_node(1.), &Context::default()).unwrap(),
            "1"
        );
    }

    #[test]
    fn serialize_negative_number() {
        assert_eq!(
            Number::new_node(-1.).serialize(&Context::default()),
            Err(SerializeNodeError::InvalidNode)
        );
    }

    #[test]
    fn serialize_number_float_dot() {
        assert_eq!(
            serialize_node(&Number::new_node(1.23), &Context::default())
                .unwrap(),
            "1.23"
        );
    }

    #[test]
    fn serialize_number_float_comma() {
        assert_eq!(
            serialize_node(
                &Number::new_node(1.23),
                &Context::new(
                    Stack::new(),
                    DecimalPlaces::from(5),
                    Language::German,
                    HostApi::empty().into()
                )
            )
            .unwrap(),
            "1,23"
        );
    }

    #[test]
    fn serialize_number_scientific_notation() {
        assert_eq!(
            serialize_node(&Number::new_node(112345678.), &Context::default())
                .unwrap(),
            "1.12346 * 10 ^ 8"
        );
    }

    #[test]
    fn serialize_number_scientific_notation_negative() {
        assert_eq!(
            serialize_node(&Number::new_node(-112345678.), &Context::default())
                .unwrap(),
            "-1.12346 * 10 ^ 8"
        );
    }
}
