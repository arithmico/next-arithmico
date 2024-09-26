use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionCall {
    pub target: Box<Node>,
    pub arguments: Vec<Node>,
}

impl FunctionCall {
    pub fn new(target: Node, arguments: Vec<Node>) -> Node {
        Node::FunctionCall(FunctionCall {
            target: target.into(),
            arguments,
        })
    }
}
