use std::collections::HashMap;

use super::{node::EditorNode, selection::Selection};

pub struct EditorState {
    nodes: HashMap<usize, Box<dyn EditorNode>>,
    root_id: usize,
    selection: Selection,
}

impl EditorState {
    pub fn node_ids(&self) -> Vec<usize> {
        self.nodes.keys().copied().collect()
    }

    pub fn node(&self, id: usize) -> Option<&Box<dyn EditorNode>> {
        self.nodes.get(&id)
    }

    pub fn selection(&self) -> Selection {
        self.selection.clone()
    }

    pub fn root_id(&self) -> usize {
        self.root_id
    }
}
