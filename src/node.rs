#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Number {
        value: f64,
    },
    Symbol {
        name: String,
    },
    Boolean {
        value: bool,
    },
    Negate {
        value: Box<Node>,
    },
    Sum {
        values: Vec<Node>,
    },
    Product {
        values: Vec<Node>,
    },
    Division {
        dividend: Box<Node>,
        divisor: Box<Node>,
    },
    Power {
        base: Box<Node>,
        exponent: Box<Node>,
    },
    Vector {
        values: Vec<Node>,
    },
    FunctionCall {
        target: Box<Node>,
        arguments: Vec<Node>,
    },
    Function {
        arguments: Vec<String>,
        expression: Box<Node>,
    },
    Definition {
        symbol: String,
        expression: Box<Node>,
    },
}
