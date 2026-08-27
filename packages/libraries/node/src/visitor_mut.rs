use std::{ops::ControlFlow, vec};

use crate::Node;

pub trait NodeVisitorMut {
    type Break;

    fn visit(&mut self, node: &mut Node) -> ControlFlow<Self::Break, ()>;
}

enum StackEntry {
    Explore(*mut Node),
    Process(*mut Node),
}

impl Node {
    pub fn visit_post_order_mut<V: NodeVisitorMut>(
        &mut self,
        visitor: &mut V,
    ) -> ControlFlow<V::Break, ()> {
        let mut stack: Vec<StackEntry> =
            vec![StackEntry::Explore(self as *mut Node)];

        while let Some(entry) = stack.pop() {
            match entry {
                StackEntry::Process(node_ptr) => {
                    // SAFETY: We process the parent only after all children have been
                    // popped from the stack and processed, ensuring no active aliases.
                    let node = unsafe { &mut *node_ptr };
                    match visitor.visit(node) {
                        ControlFlow::Continue(_) => (),
                        ControlFlow::Break(b) => return ControlFlow::Break(b),
                    }
                }
                StackEntry::Explore(node_ptr) => {
                    stack.push(StackEntry::Process(node_ptr));

                    // SAFETY: We only borrow the node to extract pointers to its children.
                    let node = unsafe { &mut *node_ptr };

                    match node {
                        Node::HostFunction(_)
                        | Node::Number(_)
                        | Node::Symbol(_)
                        | Node::Boolean(_) => (),
                        Node::Sum(sum) => {
                            stack.reserve(sum.elements.len());
                            stack.extend(
                                sum.elements
                                    .iter_mut()
                                    .rev()
                                    .map(|node| StackEntry::Explore(node)),
                            );
                        }
                        Node::Negate(negate) => {
                            stack.push(StackEntry::Explore(
                                negate.value.as_mut(),
                            ));
                        }
                        Node::Product(product) => {
                            stack.reserve(product.elements.len());
                            stack.extend(
                                product
                                    .elements
                                    .iter_mut()
                                    .rev()
                                    .map(|node| StackEntry::Explore(node)),
                            );
                        }
                        Node::Division(division) => {
                            stack.push(StackEntry::Explore(
                                division.divisor.as_mut(),
                            ));
                            stack.push(StackEntry::Explore(
                                division.dividend.as_mut(),
                            ));
                        }
                        Node::Power(power) => {
                            stack.push(StackEntry::Explore(
                                power.exponent.as_mut(),
                            ));
                            stack
                                .push(StackEntry::Explore(power.base.as_mut()));
                        }
                        Node::Tensor(tensor) => {
                            stack.reserve(tensor.elements.len());
                            stack.extend(
                                tensor
                                    .elements
                                    .iter_mut()
                                    .rev()
                                    .map(|node| StackEntry::Explore(node)),
                            );
                        }
                        Node::Function(function) => {
                            stack.push(StackEntry::Explore(
                                function.expression.as_mut(),
                            ));
                        }
                        Node::FunctionCall(function_call) => {
                            stack.reserve(function_call.arguments.len());
                            stack.extend(
                                function_call
                                    .arguments
                                    .iter_mut()
                                    .rev()
                                    .map(|node| StackEntry::Explore(node)),
                            );
                        }
                        Node::And(and) => {
                            stack.reserve(and.elements.len());
                            stack.extend(
                                and.elements
                                    .iter_mut()
                                    .rev()
                                    .map(|node| StackEntry::Explore(node)),
                            );
                        }
                        Node::Or(or) => {
                            stack.reserve(or.elements.len());
                            stack.extend(
                                or.elements
                                    .iter_mut()
                                    .rev()
                                    .map(|node| StackEntry::Explore(node)),
                            );
                        }
                        Node::Equals(relation) => {
                            stack.push(StackEntry::Explore(
                                relation.right.as_mut(),
                            ));
                            stack.push(StackEntry::Explore(
                                relation.left.as_mut(),
                            ));
                        }
                        Node::LessThan(relation) => {
                            stack.push(StackEntry::Explore(
                                relation.right.as_mut(),
                            ));
                            stack.push(StackEntry::Explore(
                                relation.left.as_mut(),
                            ));
                        }
                        Node::LessThanOrEquals(relation) => {
                            stack.push(StackEntry::Explore(
                                relation.right.as_mut(),
                            ));
                            stack.push(StackEntry::Explore(
                                relation.left.as_mut(),
                            ));
                        }
                        Node::GreaterThan(relation) => {
                            stack.push(StackEntry::Explore(
                                relation.right.as_mut(),
                            ));
                            stack.push(StackEntry::Explore(
                                relation.left.as_mut(),
                            ));
                        }
                        Node::GreaterThanOrEquals(relation) => {
                            stack.push(StackEntry::Explore(
                                relation.right.as_mut(),
                            ));
                            stack.push(StackEntry::Explore(
                                relation.left.as_mut(),
                            ));
                        }
                        Node::Definition(definition) => {
                            stack.push(StackEntry::Explore(
                                definition.expression.as_mut(),
                            ));
                        }
                        Node::Factorial(factorial) => {
                            stack.push(StackEntry::Explore(
                                factorial.value.as_mut(),
                            ));
                        }
                        Node::DataFrame(data_frame) => {
                            stack.reserve(data_frame.data.len());
                            stack.extend(
                                data_frame.data.iter_mut().rev().filter_map(
                                    |node| {
                                        Some(StackEntry::Explore(
                                            node.as_mut()?,
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
