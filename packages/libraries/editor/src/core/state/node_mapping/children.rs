use std::collections::{HashSet, VecDeque};

use web_sys::{wasm_bindgen::JsCast, Node};

use super::NodeMapping;

#[allow(dead_code)]
impl NodeMapping {
    pub fn get_children_ids(&self, node_id: usize) -> Vec<usize> {
        self.children
            .get(&node_id)
            .map(|children| children.as_ref())
            .flatten()
            .map(|children| children.clone())
            .unwrap_or_else(|| vec![])
    }

    pub fn get_all_children_ids(&self, node_id: usize) -> HashSet<usize> {
        let mut children = self
            .get_children_ids(node_id)
            .iter()
            .copied()
            .collect::<HashSet<_>>();

        let mut queue = VecDeque::<usize>::from_iter(children.iter().copied());
        while let Some(node_id) = queue.pop_front() {
            let node_children = self.get_children_ids(node_id);
            for node_child in node_children {
                queue.push_back(node_child);
                children.insert(node_child);
            }
        }

        children
    }

    pub fn has_children(&self, node_id: usize) -> bool {
        self.children
            .get(&node_id)
            .map(|children| {
                children.as_ref().map(|children| !children.is_empty())
            })
            .flatten()
            .unwrap_or(false)
    }

    pub fn supports_children(&self, node_id: usize) -> bool {
        self.get_node(node_id).expect("node").supports_children()
    }

    pub fn get_current_children_from_dom(
        &self,
        node_id: usize,
    ) -> Option<Vec<Node>> {
        self.get_dom_node(node_id)
            .map(|node| {
                node.child_nodes()
                    .values()
                    .into_iter()
                    .map(|child| child?.dyn_into::<Node>())
                    .collect::<Result<Vec<Node>, _>>()
                    .ok()
            })
            .flatten()
    }

    pub fn get_children_dom_nodes(&self, node_id: usize) -> Option<Vec<Node>> {
        self.get_children_ids(node_id)
            .into_iter()
            .map(|child_id| self.get_dom_node(child_id).cloned())
            .collect::<Option<Vec<_>>>()
    }

    pub fn get_children_count(&self, node_id: usize) -> Option<usize> {
        Some(self.children.get(&node_id)?.as_ref()?.len())
    }

    pub fn get_child_position(&self, node_id: usize) -> Option<usize> {
        let parent_id = self.parent_id(node_id)?;
        let children = self.get_children_ids(parent_id);
        children.iter().position(|child_id| child_id.eq(&node_id))
    }

    pub fn get_child_at(
        &self,
        node_id: usize,
        position: usize,
    ) -> Option<usize> {
        self.get_children_ids(node_id).get(position).copied()
    }

    pub fn get_next_sibling(&self, node_id: usize) -> Option<usize> {
        let position = self.get_child_position(node_id)?;
        let parent_id = self.parent_id(node_id)?;
        self.get_child_at(parent_id, position + 1)
    }

    pub fn get_previous_sibling(&self, node_id: usize) -> Option<usize> {
        let position = self.get_child_position(node_id)?;
        if position < 1 {
            return None;
        }
        let parent_id = self.parent_id(node_id)?;
        self.get_child_at(parent_id, position - 1)
    }
}
