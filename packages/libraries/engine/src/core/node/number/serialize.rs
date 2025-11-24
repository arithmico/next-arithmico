use crate::core::{
    get_decimal_separator, Context, Negate, Node, NormalizeNode, Number, Power,
    Product, Serialize, SerializeNodeError,
};

impl NormalizeNode for Number {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError> {
        if self.value == 0.0 {
            return Ok(Number::new(self.value));
        }

        let decimal_places = i32::from(&context.decimal_places);

        let magnitude = self.value.abs().log10().round() as i64;
        let magnitude_abs = if magnitude < 0 { -magnitude } else { magnitude };
        if magnitude_abs <= decimal_places as i64 {
            return if self.value < 0.0 {
                Ok(Negate::new(Number::new(self.value.abs())))
            } else {
                Ok(Number::new(self.value))
            };
        }

        let sign = self.value.signum();
        let factor = self.value.abs() * 10_f64.powi(-magnitude as i32);
        let scientific_notation = Product::new(vec![
            Number::new(factor).into(),
            Power::new(
                Number::new(10.0),
                if magnitude < 0 {
                    Node::from(Negate::new(Number::new(magnitude.abs() as f64)))
                } else {
                    Node::from(Number::new(magnitude.abs() as f64))
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

    use crate::core::{
        serialize_node, DecimalFormat, DecimalPlaces, HostApi, Stack,
    };

    use super::*;

    #[test]
    fn serialize_number_int() {
        assert_eq!(
            serialize_node(&Number::new(1.), &Context::default()).unwrap(),
            "1"
        );
    }

    #[test]
    fn serialize_negative_number() {
        assert_eq!(
            Number::new(-1.).serialize(&Context::default()),
            Err(SerializeNodeError::InvalidNode)
        );
    }

    #[test]
    fn serialize_number_float_dot() {
        assert_eq!(
            serialize_node(&Number::new(1.23), &Context::default()).unwrap(),
            "1.23"
        );
    }

    #[test]
    fn serialize_number_float_comma() {
        assert_eq!(
            serialize_node(
                &Number::new(1.23),
                &Context::new(
                    Stack::new(),
                    DecimalPlaces::from(5),
                    DecimalFormat::Comma,
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
            serialize_node(&Number::new(112345678.), &Context::default())
                .unwrap(),
            "1.12346 * 10 ^ 8"
        );
    }

    #[test]
    fn serialize_number_scientific_notation_negative() {
        assert_eq!(
            serialize_node(&Number::new(-112345678.), &Context::default())
                .unwrap(),
            "-1.12346 * 10 ^ 8"
        );
    }
}
