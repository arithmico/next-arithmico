use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use selection::SelectionRange;
use send_wrapper::SendWrapper;
use web_sys::Node;

use crate::{
    EditorContainerNode, EditorNode, EditorTransform, RootNode,
    utils::get_node_id,
};

pub mod commands;
pub mod queries;
pub mod selection;

#[derive(Clone)]
pub struct EditorState {
    editor_nodes: HashMap<usize, EditorNode>,
    dom_nodes: HashMap<usize, Option<SendWrapper<Node>>>,
    children: HashMap<usize, Option<Vec<usize>>>,
    parent: HashMap<usize, Option<usize>>,
    root_node_id: usize,
    modified_nodes: HashSet<usize>,
    selection: Option<SelectionRange>,
    transforms: Vec<Arc<dyn EditorTransform>>,
}

impl EditorState {
    pub fn new_with_root_node(root_node: EditorNode) -> Self {
        assert!(root_node.supports_children());
        let root_node_id = get_node_id();
        let mut state = Self {
            editor_nodes: HashMap::new(),
            dom_nodes: HashMap::new(),
            children: HashMap::new(),
            parent: HashMap::new(),
            root_node_id,
            modified_nodes: HashSet::new(),
            selection: None,
            transforms: Vec::new(),
        };
        state.set_node_entries(
            root_node_id,
            root_node,
            None,
            Some(vec![]),
            None,
        );
        state
    }
}

impl Default for EditorState {
    fn default() -> Self {
        let root_node = RootNode;
        Self::new_with_root_node(root_node.into_editor_node())
    }
}
