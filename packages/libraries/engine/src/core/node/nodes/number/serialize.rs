use crate::core::{
    context::Context,
    node::{Negate, Node, Power, Product, SerializeNode},
};

use super::Number;

impl SerializeNode for Number {
    fn transform_before_serialization(&self, context: &Context) -> Node {
        if self.value == 0.0 {
            return self.clone().into();
        }
        let decimal_places = context.settings.get_decimal_places() as i32;
        let magnitude = self.value.abs().log10().round() as i64;
        let magnitude_abs = if magnitude < 0 { -magnitude } else { magnitude };
        if magnitude_abs <= decimal_places as i64 {
            return if self.value < 0.0 {
                Negate::new(Number::new(self.value.abs())).into()
            } else {
                self.clone().into()
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
        ])
        .into();

        if sign > 0.0 {
            scientific_notation
        } else {
            Negate::new(scientific_notation).into()
        }
    }

    fn serialize(&self, context: &Context) -> String {
        if self.value == 0.0 {
            return "0".into();
        }
        if self.value < 0.0 {
            panic!("can not serialize node {}", self.value);
        }
        let decimal_places = context.settings.get_decimal_places() as usize;
        let serialized_value = format!("{:.1$}", self.value, decimal_places);
        String::from(
            serialized_value.trim_end_matches("0").trim_end_matches("."),
        )
    }
}
