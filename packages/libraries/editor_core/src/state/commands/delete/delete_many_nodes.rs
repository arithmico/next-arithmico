use std::collections::HashSet;

use crate::state::EditorState;

impl EditorState {
    /// This does not delete the root node!
    pub fn delete_many_nodes(
        &mut self,
        node_ids: HashSet<usize>,
    ) -> Result<(), crate::Error> {
        let parent_ids = node_ids
            .iter()
            .copied()
            .filter_map(|node_id| match self.get_parent_id(node_id) {
                Ok(parent_id) => Some(Ok(parent_id?)),
                Err(err) => Some(Err(err)),
            })
            .collect::<Result<HashSet<_>, _>>()?;

        let nodes_to_delete = node_ids
            .iter()
            .copied()
            .flat_map(|node_id| self.get_all_children_ids(node_id))
            .chain(node_ids.iter().copied())
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
            self.mark_node_id_as_modified(parent_id);
        }

        Ok(())
    }
}
