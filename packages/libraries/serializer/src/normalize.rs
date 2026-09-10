use node::{Negate, Node, Number, Power, Product};
use transformer::TransformNode;

use crate::Error;

pub(crate) trait NormalizeNode {
    fn normalize(self, options: crate::Options) -> Result<Node, crate::Error>;
}

impl NormalizeNode for Node {
    fn normalize(self, options: crate::Options) -> Result<Node, crate::Error> {
        self.transform(|outer_node| match outer_node {
            Node::Number(node) => {
                if node.value == 0.0 {
                    return Ok(());
                }

                let decimal_places = usize::from(options.decimal_places);

                let magnitude = node.value.abs().log10().round() as i64;
                let magnitude_abs =
                    if magnitude < 0 { -magnitude } else { magnitude };
                if magnitude_abs <= decimal_places as i64 {
                    if node.value < 0.0 {
                        *outer_node =
                            Negate::new(Number::new_node(node.value.abs()));
                    }
                    return Ok(());
                }

                let sign = node.value.signum();
                let factor = node.value.abs() * 10_f64.powi(-magnitude as i32);
                let scientific_notation = Product::new(vec![
                    Number::new_node(factor).into(),
                    Power::new(
                        Number::new_node(10.0),
                        if magnitude < 0 {
                            Node::from(Negate::new(Number::new_node(
                                magnitude.abs() as f64,
                            )))
                        } else {
                            Node::from(Number::new_node(magnitude.abs() as f64))
                        },
                    )
                    .into(),
                ]);

                *outer_node = if sign > 0.0 {
                    scientific_notation
                } else {
                    Negate::new(scientific_notation)
                };
                Ok(())
            }
            Node::HostFunction(_) => Err(Error::unsupported_node(&outer_node)),
            _ => Ok(()),
        })
    }
}
