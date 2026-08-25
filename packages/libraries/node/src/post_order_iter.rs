use std::collections::VecDeque;

use crate::Node;

impl Node {
    pub fn post_order_iter<'a>(&'a self) -> NodePostOrderIter<'a> {
        NodePostOrderIter::new(self)
    }
}

pub struct NodePostOrderIter<'a> {
    queue: VecDeque<QueueItem<'a>>,
}

#[derive(Debug, Clone, Copy)]
struct QueueItem<'a> {
    node: &'a Node,
    expanded: bool,
}

impl<'a> QueueItem<'a> {
    fn new(node: &'a Node) -> Self {
        Self {
            node,
            expanded: false,
        }
    }
}

impl<'a> NodePostOrderIter<'a> {
    pub fn new(node: &'a Node) -> Self {
        let mut queue = VecDeque::with_capacity(1);
        queue.push_front(QueueItem {
            node,
            expanded: false,
        });
        Self { queue }
    }
}

impl<'a> Iterator for NodePostOrderIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut item) = self.queue.pop_front() {
            if item.expanded {
                return Some(item.node);
            }
            item.expanded = true;
            self.queue.push_front(item);
            match item.node {
                Node::HostFunction(_)
                | Node::Number(_)
                | Node::Symbol(_)
                | Node::Boolean(_) => (),
                Node::Sum(sum) => {
                    self.queue.reserve_exact(sum.elements.len());
                    for node in sum.elements.iter().rev() {
                        self.queue.push_front(QueueItem::new(node));
                    }
                }
                Node::Negate(negate) => {
                    self.queue
                        .push_front(QueueItem::new(negate.value.as_ref()));
                }
                Node::Product(product) => {
                    self.queue.reserve_exact(product.elements.len());
                    for node in product.elements.iter().rev() {
                        self.queue.push_front(QueueItem::new(node));
                    }
                }
                Node::Division(division) => {
                    self.queue.reserve_exact(2);
                    self.queue.push_front(QueueItem::new(&division.divisor));
                    self.queue.push_front(QueueItem::new(&division.dividend));
                }
                Node::Power(power) => {
                    self.queue.reserve_exact(2);
                    self.queue.push_front(QueueItem::new(&power.exponent));
                    self.queue.push_front(QueueItem::new(&power.base));
                }
                Node::Tensor(tensor) => {
                    self.queue.reserve_exact(tensor.elements.len());
                    for node in tensor.elements.iter().rev() {
                        self.queue.push_front(QueueItem::new(node));
                    }
                }
                Node::Function(function) => {
                    self.queue.push_front(QueueItem::new(
                        function.expression.as_ref(),
                    ));
                }
                Node::FunctionCall(function_call) => {
                    self.queue.reserve_exact(function_call.arguments.len());
                    for node in function_call.arguments.iter().rev() {
                        self.queue.push_front(QueueItem::new(node));
                    }
                }
                Node::And(and) => {
                    self.queue.reserve_exact(and.elements.len());
                    for node in and.elements.iter().rev() {
                        self.queue.push_front(QueueItem::new(node));
                    }
                }
                Node::Or(or) => {
                    self.queue.reserve_exact(or.elements.len());
                    for node in or.elements.iter().rev() {
                        self.queue.push_front(QueueItem::new(node));
                    }
                }
                Node::Equals(relation) => {
                    self.queue.reserve_exact(2);
                    self.queue.push_front(QueueItem::new(&relation.right));
                    self.queue.push_front(QueueItem::new(&relation.left));
                }
                Node::LessThan(relation) => {
                    self.queue.reserve_exact(2);
                    self.queue.push_front(QueueItem::new(&relation.right));
                    self.queue.push_front(QueueItem::new(&relation.left));
                }
                Node::LessThanOrEquals(relation) => {
                    self.queue.reserve_exact(2);
                    self.queue.push_front(QueueItem::new(&relation.right));
                    self.queue.push_front(QueueItem::new(&relation.left));
                }
                Node::GreaterThan(relation) => {
                    self.queue.reserve_exact(2);
                    self.queue.push_front(QueueItem::new(&relation.right));
                    self.queue.push_front(QueueItem::new(&relation.left));
                }
                Node::GreaterThanOrEquals(relation) => {
                    self.queue.reserve_exact(2);
                    self.queue.push_front(QueueItem::new(&relation.right));
                    self.queue.push_front(QueueItem::new(&relation.left));
                }
                Node::Definition(definition) => {
                    self.queue.push_front(QueueItem::new(
                        definition.expression.as_ref(),
                    ));
                }
                Node::Factorial(factorial) => {
                    self.queue
                        .push_front(QueueItem::new(factorial.value.as_ref()));
                }
                Node::DataFrame(data_frame) => {
                    self.queue.reserve_exact(data_frame.data.len());
                    for node in data_frame.data.iter().rev() {
                        if let Some(node) = node {
                            self.queue.push_front(QueueItem::new(node));
                        }
                    }
                }
            }
        }
        None
    }
}
