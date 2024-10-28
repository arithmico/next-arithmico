use std::any::Any;

use web_sys::Node;

pub trait EditorNode: Any + 'static {
    fn create_node(&self) -> Node;
    fn supports_children(&self) -> bool;
    fn requires_update(&self, dom_node: &Node) -> bool;
    fn as_any(&self) -> Box<&dyn Any>;
    fn boxed_clone(&self) -> Box<dyn EditorNode>;
}

pub trait EditorLeafNode: Any + 'static {
    fn create_node(&self) -> Node;
    fn requires_update(&self, dom_node: &Node) -> bool;
    fn as_any(&self) -> Box<&dyn Any>;
    fn boxed_clone(&self) -> Box<dyn EditorNode>;
}

impl<T: EditorLeafNode> EditorNode for T {
    fn create_node(&self) -> Node {
        self.create_node()
    }

    fn supports_children(&self) -> bool {
        false
    }

    fn requires_update(&self, dom_node: &Node) -> bool {
        self.requires_update(dom_node)
    }

    fn as_any(&self) -> Box<&dyn Any> {
        self.as_any()
    }

    fn boxed_clone(&self) -> Box<dyn EditorNode> {
        self.boxed_clone()
    }
}

impl<T: EditorNode + 'static> From<T> for Box<dyn EditorNode> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}
