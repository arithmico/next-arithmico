use std::iter::zip;

use node::Node;

use crate::{
    Error, Options,
    seperators::{ArgumentSeperator, DecimalSeperator},
    serializer::Serializer,
};

pub(crate) trait SerializeNormalizedNode {
    fn serialize_normalized_node(
        &self,
        serializer: &mut Serializer,
        options: Options,
    ) -> Result<(), Error>;
}

impl SerializeNormalizedNode for Node {
    fn serialize_normalized_node(
        &self,
        serializer: &mut Serializer,
        options: Options,
    ) -> Result<(), Error> {
        match self {
            Node::Boolean(node) => {
                if node.value {
                    serializer.write("true");
                } else {
                    serializer.write("false");
                }
            }
            Node::Number(node) if node.value == 0.0 => {
                serializer.write("0");
            }
            Node::Number(node) if node.value < 0.0 => {
                return Err(Error::malformed_node(self));
            }
            Node::Number(node) => {
                let decimal_places = usize::from(options.decimal_places);
                let serialized_value =
                    format!("{:.1$}", node.value, decimal_places);

                let decimal_separator =
                    DecimalSeperator::from(options.language).to_string();

                let number_string = serialized_value
                    .trim_end_matches("0")
                    .trim_end_matches(".")
                    .replace(".", &decimal_separator);

                serializer.write(&number_string);
            }
            Node::Sum(node) => {
                if node.elements.len() < 2 {
                    return Err(Error::malformed_node(self));
                }
                let mut elements = node.elements.iter();
                let first = elements
                    .next()
                    .ok_or_else(|| Error::malformed_node(self))?;
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger(
                        self,
                        first,
                        |serializer| {
                            first.serialize_normalized_node(serializer, options)
                        },
                    )?;

                for element in elements {
                    match element {
                        Node::Negate(node) => {
                            serializer.write_spaced("-");
                            serializer.write_parenthesized_if_outer_binding_power_stronger_or_equal(
                                self,node.value.as_ref(),
                                |serializer| {
                                    node.value.serialize_normalized_node(
                                        serializer, options,
                                    )
                                },
                            )?;
                        }
                        node => {
                            serializer.write_spaced("+");
                            serializer.write_parenthesized_if_outer_binding_power_stronger(
                                self,node,
                                |serializer| {
                                    node.serialize_normalized_node(
                                        serializer, options,
                                    )
                                },
                            )?;
                        }
                    }
                }
            }
            Node::Negate(node) => {
                serializer.write("-");
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger_or_equal(
                        self,
                        node.value.as_ref(),
                        |serializer| {
                            node.value
                                .serialize_normalized_node(serializer, options)
                        },
                    )?;
            }
            Node::Product(node) => {
                if node.elements.len() < 2 {
                    return Err(Error::malformed_node(self));
                }
                let elements = node.elements.iter().enumerate();
                for (pos, node) in elements {
                    if pos > 0 {
                        serializer.write_spaced("*");
                    }
                    serializer
                        .write_parenthesized_if_outer_binding_power_stronger(
                            self,
                            node,
                            |serializer| {
                                node.serialize_normalized_node(
                                    serializer, options,
                                )
                            },
                        )?;
                }
            }
            Node::Division(node) => {
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger(
                        self,
                        node.dividend.as_ref(),
                        |serializer| {
                            node.dividend
                                .serialize_normalized_node(serializer, options)
                        },
                    )?;
                serializer.write_spaced("/");
                serializer.write_parenthesized_if_outer_binding_power_stronger_or_equal(
                    self,node.divisor.as_ref(),
                    |serializer| {
                        node.divisor
                            .serialize_normalized_node(serializer, options)
                    },
                )?;
            }
            Node::Power(node) => {
                serializer.write_parenthesized_if_outer_binding_power_stronger_or_equal(
                    self,node.base.as_ref(),
                    |serializer| {
                        node.base.serialize_normalized_node(serializer, options)
                    },
                )?;
                serializer.write_spaced("^");
                serializer.write_parenthesized_if_outer_binding_power_stronger_or_equal(
                    self,node.exponent.as_ref(),
                    |serializer| {
                        node.exponent
                            .serialize_normalized_node(serializer, options)
                    },
                )?
            }
            Node::Tensor(tensor) => {
                let elements = tensor.elements.iter().enumerate();
                let rank = tensor.get_rank();
                serializer.write(&"[".repeat(rank));

                for (current_inner_index, serialized_element) in elements {
                    let last_outer_index = tensor
                        .convert_to_outer_index(
                            current_inner_index.saturating_sub(1),
                        )
                        .ok_or_else(|| Error::malformed_node(self))?;
                    let current_outer_index = tensor
                        .convert_to_outer_index(current_inner_index)
                        .ok_or_else(|| Error::malformed_node(self))?;
                    let mut index_delta: Vec<_> =
                        zip(&last_outer_index, &current_outer_index)
                            .map(|(&last_index, &current_index)| {
                                last_index.abs_diff(current_index).min(1)
                            })
                            .collect();

                    index_delta.pop();
                    let delta_count = index_delta.iter().sum::<usize>();

                    serializer.write(&"]".repeat(delta_count));
                    if current_inner_index != 0 {
                        serializer.write_spaced_after(
                            &ArgumentSeperator::from(options.language)
                                .to_string(),
                        );
                    }
                    serializer.write(&"[".repeat(delta_count));
                    serialized_element
                        .serialize_normalized_node(serializer, options)?;
                }
                serializer.write(&"]".repeat(rank));
            }
            Node::Symbol(symbol) => {
                serializer.write(&symbol.name);
            }
            Node::Function(node) => {
                serializer.write("(");
                for (pos, argument) in
                    node.signature.arguments().iter().enumerate()
                {
                    if pos > 0 {
                        serializer.write_spaced_after(
                            &ArgumentSeperator::from(options.language)
                                .to_string(),
                        );
                    }
                    // TODO: decide how to serialize cardinality and node type
                    serializer.write(argument.get_name());
                }
                serializer.write(")");
                serializer.write_spaced("->");
                serializer.write_parenthesized_if_outer_binding_power_stronger_or_equal(
                    self, node.expression.as_ref(),
                    |serializer| {
                        node.expression
                            .serialize_normalized_node(serializer, options)
                    },
                )?;
            }
            Node::FunctionCall(node) => {
                serializer.write_parenthesized_if_outer_binding_power_stronger_or_equal(
                    self,node.target.as_ref(),
                    |serializer| {
                        node.target
                            .serialize_normalized_node(serializer, options)
                    },
                )?;
                serializer.write("(");
                for (pos, argument) in node.arguments.iter().enumerate() {
                    if pos > 0 {
                        serializer.write_spaced_after(
                            &ArgumentSeperator::from(options.language)
                                .to_string(),
                        );
                    }
                    argument.serialize_normalized_node(serializer, options)?;
                }
                serializer.write(")");
            }
            Node::And(node) => {
                if node.elements.len() < 2 {
                    return Err(Error::malformed_node(self));
                }
                let elements = node.elements.iter().enumerate();
                for (pos, node) in elements {
                    if pos > 0 {
                        serializer.write_spaced("&");
                    }
                    serializer
                        .write_parenthesized_if_outer_binding_power_stronger(
                            self,
                            node,
                            |serializer| {
                                node.serialize_normalized_node(
                                    serializer, options,
                                )
                            },
                        )?;
                }
            }
            Node::Or(node) => {
                if node.elements.len() < 2 {
                    return Err(Error::malformed_node(self));
                }
                let elements = node.elements.iter().enumerate();
                for (pos, node) in elements {
                    if pos > 0 {
                        serializer.write_spaced("|");
                    }
                    serializer
                        .write_parenthesized_if_outer_binding_power_stronger(
                            self,
                            node,
                            |serializer| {
                                node.serialize_normalized_node(
                                    serializer, options,
                                )
                            },
                        )?;
                }
            }
            Node::Equals(node) => {
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger_or_equal(
                        self,
                        node.left.as_ref(),
                        |serializer| {
                            node.left
                                .serialize_normalized_node(serializer, options)
                        },
                    )?;
                serializer.write_spaced("=");
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger_or_equal(
                        self,
                        node.right.as_ref(),
                        |serializer| {
                            node.right
                                .serialize_normalized_node(serializer, options)
                        },
                    )?;
            }
            Node::LessThan(node) => {
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger_or_equal(
                        self,
                        node.left.as_ref(),
                        |serializer| {
                            node.left
                                .serialize_normalized_node(serializer, options)
                        },
                    )?;
                serializer.write_spaced("<");
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger_or_equal(
                        self,
                        node.right.as_ref(),
                        |serializer| {
                            node.right
                                .serialize_normalized_node(serializer, options)
                        },
                    )?;
            }
            Node::LessThanOrEquals(node) => {
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger_or_equal(
                        self,
                        node.left.as_ref(),
                        |serializer| {
                            node.left
                                .serialize_normalized_node(serializer, options)
                        },
                    )?;
                serializer.write_spaced("<=");
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger_or_equal(
                        self,
                        node.right.as_ref(),
                        |serializer| {
                            node.right
                                .serialize_normalized_node(serializer, options)
                        },
                    )?;
            }
            Node::GreaterThan(node) => {
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger_or_equal(
                        self,
                        node.left.as_ref(),
                        |serializer| {
                            node.left
                                .serialize_normalized_node(serializer, options)
                        },
                    )?;
                serializer.write_spaced(">");
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger_or_equal(
                        self,
                        node.right.as_ref(),
                        |serializer| {
                            node.right
                                .serialize_normalized_node(serializer, options)
                        },
                    )?;
            }
            Node::GreaterThanOrEquals(node) => {
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger_or_equal(
                        self,
                        node.left.as_ref(),
                        |serializer| {
                            node.left
                                .serialize_normalized_node(serializer, options)
                        },
                    )?;
                serializer.write_spaced(">=");
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger_or_equal(
                        self,
                        node.right.as_ref(),
                        |serializer| {
                            node.right
                                .serialize_normalized_node(serializer, options)
                        },
                    )?;
            }
            Node::Definition(node) => {
                serializer.write(&node.symbol);
                match node.expression.as_ref() {
                    Node::Function(function) => {
                        serializer.write("(");
                        let arguments =
                            function.signature.arguments().iter().enumerate();
                        for (pos, argument) in arguments {
                            if pos > 0 {
                                serializer.write_spaced_after(
                                    &ArgumentSeperator::from(options.language)
                                        .to_string(),
                                );
                            }
                            serializer.write(argument.get_name());
                        }
                        serializer.write(")");
                        serializer.write_spaced(":=");
                        serializer.write_parenthesized_if_outer_binding_power_stronger_or_equal(
                        self,function.expression.as_ref(),
                        |serializer| {
                            function.expression
                                .serialize_normalized_node(serializer, options)
                        })?;
                    }
                    _ => {
                        serializer.write_spaced(":=");
                        serializer.write_parenthesized_if_outer_binding_power_stronger_or_equal(
                            self,node.expression.as_ref(),
                            |serializer| {
                                node.expression
                                    .serialize_normalized_node(serializer, options)
                            },
                        )?;
                    }
                }
            }
            Node::Factorial(factorial) => {
                serializer
                    .write_parenthesized_if_outer_binding_power_stronger(
                        self,
                        factorial.value.as_ref(),
                        |serializer| {
                            factorial
                                .value
                                .serialize_normalized_node(serializer, options)
                        },
                    )?;
                serializer.write("!");
            }
            node => return Err(Error::unsupported_node(node)),
        }

        Ok(())
    }
}
