use std::any::Any;

use web_sys::Node;

pub enum EditorNode {
    Container(Box<dyn EditorContainerNode>),
    Leaf(Box<dyn EditorLeafNode>),
}

impl EditorNode {
    pub fn supports_children(&self) -> bool {
        match self {
            EditorNode::Container(_) => true,
            EditorNode::Leaf(_) => false,
        }
    }

    pub fn as_any(&self) -> Box<&dyn Any> {
        match self {
            EditorNode::Container(container) => container.as_any(),
            EditorNode::Leaf(leaf) => leaf.as_any(),
        }
    }

    pub fn create_node(&self) -> Node {
        match self {
            EditorNode::Container(container) => container.create_node(),
            EditorNode::Leaf(leaf) => leaf.create_node(),
        }
    }

    pub fn requires_update(&self, dom_node: &Node) -> bool {
        match self {
            EditorNode::Container(container) => {
                container.requires_update(dom_node)
            }
            EditorNode::Leaf(leaf) => leaf.requires_update(dom_node),
        }
    }
}

pub trait EditorContainerNode: Any + 'static {
    fn create_node(&self) -> Node;
    fn requires_update(&self, dom_node: &Node) -> bool;
    fn as_any(&self) -> Box<&dyn Any>;
    fn boxed_clone(&self) -> Box<dyn EditorContainerNode>;
    fn into_editor_node(self) -> EditorNode;
}

pub trait EditorLeafNode: Any + 'static {
    fn create_node(&self) -> Node;
    fn requires_update(&self, dom_node: &Node) -> bool;
    fn as_any(&self) -> Box<&dyn Any>;
    fn boxed_clone(&self) -> Box<dyn EditorLeafNode>;
    fn into_editor_node(self) -> EditorNode;
    fn length(&self) -> usize;
    fn slice(&self, start: usize, end: usize) -> EditorNode;
}
