use std::collections::{HashSet, VecDeque};

use crate::core::state::EditorState;

impl EditorState {
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
}
