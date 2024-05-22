use crate::core::node::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    pub arguments: Vec<String>,
    pub expression: Box<Node>,
}

impl Function {
    pub fn new<T: Into<Node>>(
        arguments: Vec<String>,
        expression: T,
    ) -> Function {
        Function {
            arguments: arguments.into(),
            expression: expression.into().into(),
        }
    }
}
