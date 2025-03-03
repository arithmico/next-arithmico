use std::{any::Any, sync::Arc};

use web_sys::Node;

#[derive(Clone, Debug)]
pub enum EditorNode {
    Container(Arc<dyn EditorContainerNode>),
    Leaf(Arc<dyn EditorLeafNode>),
}

impl EditorNode {
    pub fn supports_children(&self) -> bool {
        match self {
            EditorNode::Container(_) => true,
            EditorNode::Leaf(_) => false,
        }
    }

    pub fn as_any(&self) -> Arc<&dyn Any> {
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

pub trait EditorContainerNode: std::fmt::Debug + Any + 'static {
    fn create_node(&self) -> Node;
    fn requires_update(&self, dom_node: &Node) -> bool;
    fn as_any(&self) -> Arc<&dyn Any>;
    fn arc_clone(&self) -> Arc<dyn EditorContainerNode>;
    fn into_editor_node(self) -> EditorNode;
    fn delete_if_empty(&self) -> bool;
}

pub trait EditorLeafNode: std::fmt::Debug + Any + 'static {
    fn create_node(&self) -> Node;
    fn requires_update(&self, dom_node: &Node) -> bool;
    fn as_any(&self) -> Arc<&dyn Any>;
    fn arc_clone(&self) -> Arc<dyn EditorLeafNode>;
    fn into_editor_node(self) -> EditorNode;
    fn length(&self) -> usize;
    fn slice(&self, start: usize, end: usize) -> EditorNode;
    fn get_whitespaces(&self) -> Vec<(usize, usize)>;
    fn serialize(&self) -> Option<String>;
}
