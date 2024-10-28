use std::collections::VecDeque;

use super::NodeMapping;

impl NodeMapping {
    pub fn get_ordered_node_ids(&self) -> Vec<usize> {
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

    pub fn get_nodes_between(
        &self,
        from_node_id: usize,
        to_node_id: usize,
    ) -> Vec<usize> {
        let node_ids = self.get_ordered_node_ids();
        let from = node_ids
            .iter()
            .position(|node_id| *node_id == from_node_id)
            .expect("from position");
        let to = node_ids
            .iter()
            .position(|node_id| *node_id == to_node_id)
            .expect("to position");
        let start = from.min(to);
        let end = from.max(to);
        node_ids
            .into_iter()
            .enumerate()
            .filter_map(|(position, node_id)| {
                if position > start && position < end {
                    Some(node_id)
                } else {
                    None
                }
            })
            .collect()
    }
}
