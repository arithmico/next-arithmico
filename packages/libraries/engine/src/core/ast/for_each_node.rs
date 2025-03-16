use crate::core::Node;

impl Node {
    pub fn for_each_node<F>(&self, f: F)
    where
        F: Fn(&Node) + Copy,
    {
        match self {
            // leaves
            Node::Boolean(_)
            | Node::Number(_)
            | Node::Symbol(_)
            | Node::HostFunction(_) => (),

            // nodes
            Node::Sum(sum) => {
                sum.elements.iter().for_each(|node| node.for_each_node(f));
            }
            Node::Product(product) => {
                product
                    .elements
                    .iter()
                    .for_each(|node| node.for_each_node(f));
            }

            Node::Tensor(tensor) => {
                tensor
                    .elements
                    .iter()
                    .for_each(|node| node.for_each_node(f));
            }
            Node::And(and) => {
                and.elements.iter().for_each(|node| node.for_each_node(f));
            }
            Node::Or(or) => {
                or.elements.iter().for_each(|node| node.for_each_node(f));
            }
            Node::Negate(negate) => {
                negate.value.for_each_node(f);
            }
            Node::Division(division) => {
                division.dividend.for_each_node(f);
                division.divisor.for_each_node(f);
            }
            Node::Power(power) => {
                power.base.for_each_node(f);
                power.exponent.for_each_node(f);
            }
            Node::Function(function) => {
                function.expression.for_each_node(f);
            }
            Node::FunctionCall(function_call) => {
                function_call.target.for_each_node(f);
                function_call
                    .arguments
                    .iter()
                    .for_each(|node| node.for_each_node(f));
            }
            Node::Equals(equals) => {
                equals.left.for_each_node(f);
                equals.right.for_each_node(f);
            }
            Node::LessThan(less_than) => {
                less_than.left.for_each_node(f);
                less_than.right.for_each_node(f);
            }
            Node::LessThanOrEquals(less_than_or_equals) => {
                less_than_or_equals.left.for_each_node(f);
                less_than_or_equals.right.for_each_node(f);
            }
            Node::GreaterThan(greater_than) => {
                greater_than.left.for_each_node(f);
                greater_than.right.for_each_node(f);
            }
            Node::GreaterThanOrEquals(greater_than_or_equals) => {
                greater_than_or_equals.left.for_each_node(f);
                greater_than_or_equals.right.for_each_node(f);
            }
            Node::Definition(definition) => {
                definition.expression.for_each_node(f);
            }
        }
        f(self);
    }

    pub fn for_each_node_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Node) + Copy,
    {
        match self {
            // leaves
            Node::Boolean(_)
            | Node::Number(_)
            | Node::Symbol(_)
            | Node::HostFunction(_) => (),

            // nodes
            Node::Sum(sum) => {
                sum.elements
                    .iter_mut()
                    .for_each(|node| node.for_each_node_mut(f));
            }
            Node::Product(product) => {
                product
                    .elements
                    .iter_mut()
                    .for_each(|node| node.for_each_node_mut(f));
            }

            Node::Tensor(tensor) => {
                tensor
                    .elements
                    .iter_mut()
                    .for_each(|node| node.for_each_node_mut(f));
            }
            Node::And(and) => {
                and.elements
                    .iter_mut()
                    .for_each(|node| node.for_each_node_mut(f));
            }
            Node::Or(or) => {
                or.elements
                    .iter_mut()
                    .for_each(|node| node.for_each_node_mut(f));
            }
            Node::Negate(negate) => {
                negate.value.for_each_node_mut(f);
            }
            Node::Division(division) => {
                division.dividend.for_each_node_mut(f);
                division.divisor.for_each_node_mut(f);
            }
            Node::Power(power) => {
                power.base.for_each_node_mut(f);
                power.exponent.for_each_node_mut(f);
            }
            Node::Function(function) => {
                function.expression.for_each_node_mut(f);
            }
            Node::FunctionCall(function_call) => {
                function_call.target.for_each_node_mut(f);
                function_call
                    .arguments
                    .iter_mut()
                    .for_each(|node| node.for_each_node_mut(f));
            }
            Node::Equals(equals) => {
                equals.left.for_each_node_mut(f);
                equals.right.for_each_node_mut(f);
            }
            Node::LessThan(less_than) => {
                less_than.left.for_each_node_mut(f);
                less_than.right.for_each_node_mut(f);
            }
            Node::LessThanOrEquals(less_than_or_equals) => {
                less_than_or_equals.left.for_each_node_mut(f);
                less_than_or_equals.right.for_each_node_mut(f);
            }
            Node::GreaterThan(greater_than) => {
                greater_than.left.for_each_node_mut(f);
                greater_than.right.for_each_node_mut(f);
            }
            Node::GreaterThanOrEquals(greater_than_or_equals) => {
                greater_than_or_equals.left.for_each_node_mut(f);
                greater_than_or_equals.right.for_each_node_mut(f);
            }
            Node::Definition(definition) => {
                definition.expression.for_each_node_mut(f);
            }
        }
        f(self);
    }
}
