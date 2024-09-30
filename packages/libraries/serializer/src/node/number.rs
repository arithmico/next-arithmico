use ast::{Negate, Node, Number, Power, Product};

use crate::{
    decimal_separator::get_decimal_separator, error::SerializeNodeError,
    serialize_node::SerializeNode,
    serialize_node_options::SerializeNodeOptions,
    serialize_node_utils::SerializeNodeUtils,
};

impl SerializeNodeUtils for Number {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError> {
        if self.value == 0.0 {
            return Ok(Number::new(self.value));
        }

        let decimal_places = i32::from(&options.decimal_places);

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

impl SerializeNode for Number {
    fn serialize(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<String, SerializeNodeError> {
        if self.value == 0.0 {
            return Ok("0".into());
        }
        if self.value < 0.0 {
            return Err(SerializeNodeError::InvalidNode);
        }

        let decimal_places = usize::from(&options.decimal_places);
        let serialized_value = format!("{:.1$}", self.value, decimal_places);

        Ok(String::from(
            serialized_value
                .trim_end_matches("0")
                .trim_end_matches(".")
                .replace(".", &get_decimal_separator(options)),
        ))
    }
}
