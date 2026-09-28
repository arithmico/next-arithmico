use std::{ops::ControlFlow, vec};

use crate::Node;

pub trait NodeVisitor<'a> {
    type Break;

    fn visit(&mut self, node: &'a Node) -> ControlFlow<Self::Break, ()>;
}

enum StackEntry<'a> {
    Explore(&'a Node),
    Process(&'a Node),
}

impl Node {
    pub fn visit_post_order<'a, V: NodeVisitor<'a>>(
        &'a self,
        visitor: &mut V,
    ) -> ControlFlow<V::Break, ()> {
        let mut stack: Vec<StackEntry<'a>> = vec![StackEntry::Explore(self)];

        while let Some(entry) = stack.pop() {
            match entry {
                StackEntry::Process(node) => match visitor.visit(node) {
                    ControlFlow::Continue(_) => (),
                    ControlFlow::Break(b) => return ControlFlow::Break(b),
                },
                StackEntry::Explore(node) => {
                    stack.push(StackEntry::Process(node));
                    match node {
                        Node::HostFunction(_)
                        | Node::Number(_)
                        | Node::Symbol(_)
                        | Node::Boolean(_) => (),
                        Node::Sum(sum) => {
                            stack.reserve(sum.elements.len());
                            stack.extend(
                                sum.elements
                                    .iter()
                                    .rev()
                                    .map(StackEntry::Explore),
                            );
                        }
                        Node::Negate(negate) => {
                            stack.push(StackEntry::Explore(&negate.value));
                        }
                        Node::Product(product) => {
                            stack.reserve(product.elements.len());
                            stack.extend(
                                product
                                    .elements
                                    .iter()
                                    .rev()
                                    .map(StackEntry::Explore),
                            );
                        }
                        Node::Division(division) => {
                            stack.push(StackEntry::Explore(&division.divisor));
                            stack.push(StackEntry::Explore(&division.dividend));
                        }
                        Node::Power(power) => {
                            stack.push(StackEntry::Explore(&power.exponent));
                            stack.push(StackEntry::Explore(&power.base));
                        }
                        Node::Tensor(tensor) => {
                            stack.reserve(tensor.elements.len());
                            stack.extend(
                                tensor
                                    .elements
                                    .iter()
                                    .rev()
                                    .map(StackEntry::Explore),
                            );
                        }
                        Node::Function(function) => {
                            stack.push(StackEntry::Explore(
                                &function.expression,
                            ));
                        }
                        Node::FunctionCall(function_call) => {
                            stack.reserve(function_call.arguments.len());
                            stack.extend(
                                function_call
                                    .arguments
                                    .iter()
                                    .rev()
                                    .map(StackEntry::Explore),
                            );
                        }
                        Node::And(and) => {
                            stack.reserve(and.elements.len());
                            stack.extend(
                                and.elements
                                    .iter()
                                    .rev()
                                    .map(StackEntry::Explore),
                            );
                        }
                        Node::Or(or) => {
                            stack.reserve(or.elements.len());
                            stack.extend(
                                or.elements
                                    .iter()
                                    .rev()
                                    .map(StackEntry::Explore),
                            );
                        }
                        Node::Equals(relation) => {
                            stack.push(StackEntry::Explore(&relation.right));
                            stack.push(StackEntry::Explore(&relation.left));
                        }
                        Node::LessThan(relation) => {
                            stack.push(StackEntry::Explore(&relation.right));
                            stack.push(StackEntry::Explore(&relation.left));
                        }
                        Node::LessThanOrEquals(relation) => {
                            stack.push(StackEntry::Explore(&relation.right));
                            stack.push(StackEntry::Explore(&relation.left));
                        }
                        Node::GreaterThan(relation) => {
                            stack.push(StackEntry::Explore(&relation.right));
                            stack.push(StackEntry::Explore(&relation.left));
                        }
                        Node::GreaterThanOrEquals(relation) => {
                            stack.push(StackEntry::Explore(&relation.right));
                            stack.push(StackEntry::Explore(&relation.left));
                        }
                        Node::Definition(definition) => {
                            stack.push(StackEntry::Explore(
                                &definition.expression,
                            ));
                        }
                        Node::Factorial(factorial) => {
                            stack.push(StackEntry::Explore(&factorial.value));
                        }
                        Node::DataFrame(data_frame) => {
                            stack.reserve(data_frame.data.len());
                            stack.extend(
                                data_frame.data.iter().rev().filter_map(
                                    |node| {
                                        Some(StackEntry::Explore(
                                            node.as_ref()?,
                                        ))
                                    },
                                ),
                            );
                        }
                    }
                }
            }
        }

        ControlFlow::Continue(())
    }
}
