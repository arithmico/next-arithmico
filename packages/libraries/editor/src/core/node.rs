use web_sys::Node;

pub trait EditorNode {
    fn node_id(&self) -> usize;

    fn set_node_id(&mut self, id: usize);

    fn content_length(&self) -> usize;

    fn children_ids(&self) -> &[usize];

    fn parent_id(&self) -> Option<usize>;

    fn create_node(&self) -> Node;

    fn requires_update(
        &self,
        previous_node: Box<dyn EditorNode>,
        dom_node: Node,
    ) -> bool;
}
