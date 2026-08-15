use std::collections::BTreeSet;

use crate::Node;

impl Node {
    /// Returns the unique symbol names contained in `node`.
    pub fn get_symbol_names(&self) -> BTreeSet<&str> {
        let mut names = BTreeSet::new();

        collect_symbol_names(self, &mut names);

        names
    }
}

fn collect_symbol_names<'a>(node: &'a Node, names: &mut BTreeSet<&'a str>) {
    match node {
        Node::Boolean(_) => {}
        Node::Sum(sum) => {
            for child in &sum.elements {
                collect_symbol_names(child, names);
            }
        }
        Node::Negate(negate) => {
            collect_symbol_names(&negate.value, names);
        }
        Node::Product(product) => {
            for child in &product.elements {
                collect_symbol_names(child, names);
            }
        }
        Node::Division(div) => {
            collect_symbol_names(&div.dividend, names);
            collect_symbol_names(&div.divisor, names);
        }
        Node::Power(power) => {
            collect_symbol_names(&power.base, names);
            collect_symbol_names(&power.exponent, names);
        }
        Node::Tensor(tensor) => {
            for element in &tensor.elements {
                collect_symbol_names(element, names);
            }
        }
        Node::Number(_) => {}
        Node::Symbol(symbol) => {
            names.insert(symbol.name.as_str());
        }
        Node::Function(function) => {
            collect_symbol_names(&function.expression, names);
        }
        Node::FunctionCall(call) => {
            collect_symbol_names(&call.target, names);

            for argument in &call.arguments {
                collect_symbol_names(argument, names);
            }
        }
        Node::And(and) => {
            for element in &and.elements {
                collect_symbol_names(element, names);
            }
        }
        Node::Or(or) => {
            for element in &or.elements {
                collect_symbol_names(element, names);
            }
        }
        Node::Equals(equals) => {
            collect_symbol_names(&equals.left, names);
            collect_symbol_names(&equals.right, names);
        }
        Node::LessThan(less_than) => {
            collect_symbol_names(&less_than.left, names);
            collect_symbol_names(&less_than.left, names);
        }
        Node::LessThanOrEquals(less_than_or_equals) => {
            collect_symbol_names(&less_than_or_equals.left, names);
            collect_symbol_names(&less_than_or_equals.left, names);
        }
        Node::GreaterThan(greater_than) => {
            collect_symbol_names(&greater_than.left, names);
            collect_symbol_names(&greater_than.left, names);
        }
        Node::GreaterThanOrEquals(greater_than_or_equals) => {
            collect_symbol_names(&greater_than_or_equals.left, names);
            collect_symbol_names(&greater_than_or_equals.left, names);
        }
        Node::HostFunction(_) => {}
        Node::Definition(def) => {
            names.insert(def.symbol.as_str());
            collect_symbol_names(&def.expression, names);
        }
        Node::Factorial(factorial) => {
            collect_symbol_names(&factorial.value, names);
        }
        Node::DataFrame(dataframe) => {
            for element in &dataframe.data {
                if let Some(element) = element {
                    collect_symbol_names(element, names);
                }
            }
        }
    }
}
