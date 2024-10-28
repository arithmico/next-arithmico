use std::collections::HashSet;

use super::NodeMapping;

impl NodeMapping {
    fn delete_node_entries(&mut self, node_id: usize) {
        self.editor_nodes.remove(&node_id);
        self.dom_nodes.remove(&node_id);
        self.children.remove(&node_id);
        self.parent.remove(&node_id);
    }

    pub fn delete_node(&mut self, node_id: usize) {
        let parent_id =
            self.parent_id(node_id).expect("don't delete the root node");
        let mut nodes_to_remove = self.get_all_children_ids(node_id);
        nodes_to_remove.insert(node_id);
        for child_id in nodes_to_remove {
            self.delete_node_entries(child_id);
        }
        let new_children = self
            .get_children_ids(parent_id)
            .into_iter()
            .filter(|child_id| *child_id != node_id)
            .collect::<Vec<_>>();
        self.children.insert(parent_id, Some(new_children));
        self.modified_nodes.insert(parent_id);
    }

    pub fn delete_many_nodes(&mut self, node_ids: HashSet<usize>) {
        let parent_ids = node_ids
            .iter()
            .copied()
            .map(|node_id| {
                self.parent_id(node_id).expect("don't delete the root node")
            })
            .collect::<HashSet<_>>();

        let nodes_to_delete = node_ids
            .iter()
            .copied()
            .flat_map(|node_id| self.get_all_children_ids(node_id))
            .collect::<HashSet<_>>();

        let affected_parent_ids = parent_ids
            .difference(&nodes_to_delete)
            .copied()
            .collect::<HashSet<_>>();

        for node_id in nodes_to_delete.iter().copied() {
            self.delete_node_entries(node_id);
        }

        for parent_id in affected_parent_ids {
            let new_children = self
                .get_children_ids(parent_id)
                .into_iter()
                .filter(|child_id| !nodes_to_delete.contains(child_id))
                .collect::<Vec<_>>();
            self.children.insert(parent_id, Some(new_children));
            self.modified_nodes.insert(parent_id);
        }
    }
}
