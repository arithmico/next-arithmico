use node::Node;

pub trait NodeConverter<Output> {
    fn convert_node(&self, node: &Node) -> Output;
}
