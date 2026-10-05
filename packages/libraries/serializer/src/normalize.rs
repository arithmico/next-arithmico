use common::NumberRepresentation;
use math_utils::integer_ratio_with_limit_denominator;
use node::{Division, Negate, Node, Number, Power, Product, Sum};
use transformer::TransformNode;

use crate::Error;

pub(crate) trait NormalizeNode {
    fn normalize(self, options: crate::Options) -> Result<Node, crate::Error>;
}

impl NormalizeNode for Node {
    fn normalize(self, options: crate::Options) -> Result<Node, crate::Error> {
        self.transform(|outer_node| match outer_node {
            Node::Number(node) => {
                *outer_node = normalize_number(node, options)?;
                Ok(())
            }
            Node::HostFunction(_) => Err(Error::unsupported_node(outer_node)),
            _ => Ok(()),
        })
    }
}

fn normalize_number(
    number: &Number,
    options: crate::Options,
) -> Result<Node, crate::Error> {
    match options.number_representation {
        NumberRepresentation::Number => {
            if number.value == 0.0 {
                return Ok(Number::new_node(number.value));
            }

            let decimal_places = usize::from(options.decimal_places);

            let magnitude = number.value.abs().log10().round() as i64;
            let magnitude_abs =
                if magnitude < 0 { -magnitude } else { magnitude };
            if magnitude_abs <= decimal_places as i64 {
                return if number.value < 0.0 {
                    Ok(Negate::new(Number::new_node(number.value.abs())))
                } else {
                    Ok(Number::new_node(number.value))
                };
            }

            let sign = number.value.signum();
            let factor = number.value.abs() * 10_f64.powi(-magnitude as i32);
            let scientific_notation = Product::new(vec![
                Number::new_node(factor),
                Power::new(
                    Number::new_node(10.0),
                    if magnitude < 0 {
                        Negate::new(Number::new_node(magnitude.abs() as f64))
                    } else {
                        Number::new_node(magnitude.abs() as f64)
                    },
                ),
            ]);

            Ok(if sign > 0.0 {
                scientific_notation
            } else {
                Negate::new(scientific_notation)
            })
        }

        NumberRepresentation::Fraction => {
            if let Some((num, denom)) =
                integer_ratio_with_limit_denominator(number.value, None)
            {
                let is_negative = num.is_negative();
                let num = num.unsigned_abs();

                if denom == 1 {
                    return if is_negative {
                        Ok(Negate::new(Number::new_node(num as f64)))
                    } else {
                        Ok(Number::new_node(num as f64))
                    };
                }

                let fraction = Division::new(
                    Number::new_node(num as f64),
                    Number::new_node(denom as f64),
                );

                return if is_negative {
                    Ok(Negate::new(fraction))
                } else {
                    Ok(fraction)
                };
            }

            // Fallback
            if number.value < 0.0 {
                Ok(Negate::new(Number::new_node(number.value.abs())))
            } else {
                Ok(Number::new_node(number.value))
            }
        }

        NumberRepresentation::MixedFraction => {
            if let Some((num, denom)) =
                integer_ratio_with_limit_denominator(number.value, None)
            {
                let is_negative = num.is_negative();
                let abs_num = num.unsigned_abs();

                let whole = abs_num / denom;
                let remainder = abs_num % denom;

                let mixed_fraction = if remainder == 0 {
                    Number::new_node(whole as f64)
                } else if whole == 0 {
                    Division::new(
                        Number::new_node(remainder as f64),
                        Number::new_node(denom as f64),
                    )
                } else {
                    Sum::new(vec![
                        Number::new_node(whole as f64),
                        Division::new(
                            Number::new_node(remainder as f64),
                            Number::new_node(denom as f64),
                        ),
                    ])
                };

                return if is_negative {
                    Ok(Negate::new(mixed_fraction))
                } else {
                    Ok(mixed_fraction)
                };
            }

            // Fallback
            if number.value < 0.0 {
                Ok(Negate::new(Number::new_node(number.value.abs())))
            } else {
                Ok(Number::new_node(number.value))
            }
        }
    }
}
