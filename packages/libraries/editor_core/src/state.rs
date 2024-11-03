use std::collections::{HashMap, HashSet};

use selection::SelectionRange;
use web_sys::Node;

use crate::{utils::get_node_id, EditorNode, RootNode};

pub mod commands;
pub mod queries;
pub mod selection;

pub struct EditorState {
    editor_nodes: HashMap<usize, Box<dyn EditorNode>>,
    dom_nodes: HashMap<usize, Option<Node>>,
    children: HashMap<usize, Option<Vec<usize>>>,
    parent: HashMap<usize, Option<usize>>,
    root_node_id: usize,
    modified_nodes: HashSet<usize>,
    selection: Option<SelectionRange>,
}

impl EditorState {
    pub fn new_with_root_node(root_node: Box<dyn EditorNode>) -> Self {
        assert!(root_node.supports_children());
        let root_node_id = get_node_id();
        let mut mapping = Self {
            editor_nodes: HashMap::new(),
            dom_nodes: HashMap::new(),
            children: HashMap::new(),
            parent: HashMap::new(),
            root_node_id,
            modified_nodes: HashSet::new(),
            selection: None,
        };
        mapping.set_node_entries(
            root_node_id,
            root_node,
            None,
            Some(vec![]),
            None,
        );
        mapping
    }

    pub fn new() -> Self {
        let root_node = RootNode::new();
        Self::new_with_root_node(root_node.into())
    }
}
