use node_mapping::NodeMapping;
use web_sys::Node;

use super::{node::EditorNode, EditorCommand};
use crate::node::RootNode;
pub use selection::*;

mod node_mapping;
mod selection;

#[allow(dead_code)]
pub struct EditorState {
    node_mapping: NodeMapping,
}

impl EditorState {
    pub fn new() -> Self {
        let root_node = RootNode::new();
        EditorState {
            node_mapping: NodeMapping::new(root_node.into()),
        }
    }

    pub fn mount_to_root(&mut self, dom_node: Node) {
        self.node_mapping
            .set_dom_node(self.node_mapping.get_root_id(), dom_node);
        self.node_mapping.update_dom();
    }

    pub fn insert_node(
        &mut self,
        node: Box<dyn EditorNode>,
        parent_id: Option<usize>,
        position: Option<usize>,
    ) -> usize {
        self.node_mapping.insert_node(node, parent_id, position)
    }

    pub fn update_selection(&mut self) {
        self.node_mapping.read_selection_from_dom();
    }

    pub fn update_dom(&mut self) {
        self.node_mapping.update_dom();
    }

    pub fn node_mapping(&self) -> &NodeMapping {
        &self.node_mapping
    }

    pub fn node_mapping_mut(&mut self) -> &mut NodeMapping {
        &mut self.node_mapping
    }

    pub fn execute_command(&mut self, command: Box<dyn EditorCommand>) {
        command.apply(self);
        self.node_mapping.update_dom();
        self.node_mapping.write_selection_to_dom();
    }
}
