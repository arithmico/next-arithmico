use std::collections::{HashMap, HashSet};

use web_sys::Node;

use crate::{core::EditorNode, node::RootNode, utils::get_node_id};

use super::SelectionRange;

pub mod children;
pub mod delete_node;
pub mod execute_command;
pub mod get_node;
pub mod get_nodes_between;
pub mod get_parent_node;
pub mod get_root_id;
pub mod insert_node;
mod mark_as_modified;
pub mod mount_root;
pub mod replace_node;
pub mod selection;
pub mod set_dom_node;
mod set_node_entries;
pub mod update_dom;

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
