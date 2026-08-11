use node::{Negate, Node, Number, Power, Product};
use node_transform::TransformNode;

use crate::Error;

pub(crate) trait NormalizeNode {
    fn normalize(self, options: crate::Options) -> Result<Node, crate::Error>;
}

impl NormalizeNode for Node {
    fn normalize(self, options: crate::Options) -> Result<Node, crate::Error> {
        self.transform(|node| match node {
            Node::Number(node) => {
                if node.value == 0.0 {
                    return Ok(Number::new_node(node.value));
                }

                let decimal_places = usize::from(options.decimal_places);

                let magnitude = node.value.abs().log10().round() as i64;
                let magnitude_abs =
                    if magnitude < 0 { -magnitude } else { magnitude };
                if magnitude_abs <= decimal_places as i64 {
                    return if node.value < 0.0 {
                        Ok(Negate::new(Number::new_node(node.value.abs())))
                    } else {
                        Ok(Number::new_node(node.value))
                    };
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

                if sign > 0.0 {
                    Ok(scientific_notation)
                } else {
                    Ok(Negate::new(scientific_notation))
                }
            }
            Node::HostFunction(node) => {
                Err(Error::unsupported_node(&Node::HostFunction(node)))
            }
            node => Ok(node),
        })
    }
}
