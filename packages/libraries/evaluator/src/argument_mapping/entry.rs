use node::Node;

#[derive(Debug, Clone)]
pub enum Entry {
    Value(Node),
    List(Vec<Node>),
    None,
}
