use std::collections::VecDeque;

use crate::state::EditorState;

impl EditorState {
    pub fn get_all_node_ids_in_order(&self) -> Vec<usize> {
        let mut node_ids = Vec::<usize>::new();
        let mut queue = VecDeque::from([self.get_root_id()]);
        while let Some(node_id) = queue.pop_front() {
            node_ids.push(node_id);
            let children = self.get_children_ids(node_id);
            for child in children {
                queue.push_front(child);
            }
        }
        node_ids
    }
}
