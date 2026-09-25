use std::collections::HashSet;

use web_sys::Node;

use crate::state::EditorState;

impl EditorState {
    fn read_child_from_dom(
        &self,
        node_id: usize,
        position: usize,
    ) -> Option<Node> {
        let node = self.get_dom_node(node_id)?;
        node.child_nodes().get(position as u32)
    }

    fn find_node_ids_without_dom_node(&self) -> HashSet<usize> {
        self.modified_nodes
            .iter()
            .copied()
            .filter(|node_id| self.get_dom_node(*node_id).is_none())
            .collect::<HashSet<_>>()
    }

    /// creates the missing dom nodes and returns a set of all directly affected parent node ids
    fn creating_missing_dom_nodes(&mut self) -> HashSet<usize> {
        let node_ids_without_dom_node = self.find_node_ids_without_dom_node();

        for node_id in node_ids_without_dom_node.iter().copied() {
            if let Some(node) = self.get_node(node_id) {
                self.set_dom_node(node_id, node.create_node());
            }
        }
        node_ids_without_dom_node
    }

    fn get_child_pair_at(
        &self,
        node_id: usize,
        position: usize,
    ) -> (Option<Node>, Option<Node>) {
        let expected_child = self.get_child_dom_node_at(node_id, position);
        let current_child = self.read_child_from_dom(node_id, position);
        (current_child, expected_child)
    }

    fn update_children_for_node(&self, node_id: usize) {
        let node = self.get_dom_node(node_id).expect("parent dom node");
        let mut position = 0;
        loop {
            match self.get_child_pair_at(node_id, position) {
                (Some(current_child), Some(expected_child)) => {
                    if current_child != expected_child {
                        node.replace_child(&expected_child, &current_child)
                            .expect("replace");
                    }
                }
                (Some(current_child), None) => {
                    node.remove_child(&current_child).expect("remove");
                    position = position.saturating_sub(1);
                }
                (None, Some(expected_child)) => {
                    node.append_child(&expected_child).expect("append");
                }
                (None, None) => {
                    break;
                }
            }
            position += 1;
        }
    }

    fn update_children_for_nodes(&self, affected_node_ids: &HashSet<usize>) {
        for node_id in affected_node_ids.iter().copied() {
            self.update_children_for_node(node_id);
        }
    }

    fn replace_nodes_that_require_update(&mut self) -> HashSet<usize> {
        let replace_node_ids = self
            .modified_nodes
            .iter()
            .copied()
            .filter(|node_id| {
                let editor_node = self.get_node(*node_id).expect("editor_node");
                let dom_node = self.get_dom_node(*node_id).expect("dom node");
                editor_node.requires_update(&dom_node)
            })
            .collect::<HashSet<_>>();

        for node_id in replace_node_ids.iter().copied() {
            let editor_node = self.get_node(node_id).expect("editor_node");
            self.set_dom_node(node_id, editor_node.create_node());
        }

        replace_node_ids
    }

    pub fn update_dom(&mut self) {
        let created_node_ids = self.creating_missing_dom_nodes();
        let replaced_node_ids = self.replace_nodes_that_require_update();
        let affected_parent_ids = created_node_ids
            .into_iter()
            .chain(replaced_node_ids)
            .filter_map(|node_id| self.get_parent_id(node_id))
            .collect::<HashSet<_>>()
            .union(&self.modified_nodes)
            .copied()
            .filter(|node_id| self.is_container_node(*node_id))
            .collect::<HashSet<_>>();

        self.update_children_for_nodes(&affected_parent_ids);
        self.modified_nodes.clear();
    }
}
