use std::collections::HashSet;

use web_sys::Node;

use crate::state::EditorState;

impl EditorState {
    fn read_child_from_dom(
        &self,
        node_id: usize,
        position: usize,
    ) -> Result<Node, crate::Error> {
        let node = self.get_dom_node(node_id)?;
        node.child_nodes()
            .get(position as u32)
            .ok_or_else(|| crate::Error::MissingDomNode)
    }

    fn find_node_ids_without_dom_node(&self) -> HashSet<usize> {
        self.modified_nodes
            .iter()
            .copied()
            .filter(|node_id| self.get_dom_node(*node_id).is_err())
            .collect::<HashSet<_>>()
    }

    /// creates the missing dom nodes and returns a set of all directly affected parent node ids
    fn creating_missing_dom_nodes(
        &mut self,
    ) -> Result<HashSet<usize>, crate::Error> {
        let node_ids_without_dom_node = self.find_node_ids_without_dom_node();

        for node_id in node_ids_without_dom_node.iter().copied() {
            if let Ok(node) = self.get_node(node_id) {
                self.set_dom_node(node_id, node.create_node()?);
            }
        }
        Ok(node_ids_without_dom_node)
    }

    fn get_child_pair_at(
        &self,
        node_id: usize,
        position: usize,
    ) -> (Option<Node>, Option<Node>) {
        let expected_child = self.get_child_dom_node_at(node_id, position).ok();
        let current_child = self.read_child_from_dom(node_id, position).ok();
        (current_child, expected_child)
    }

    fn update_children_for_node(
        &self,
        node_id: usize,
    ) -> Result<(), crate::Error> {
        let node = self.get_dom_node(node_id)?;
        let mut position = 0;
        loop {
            match self.get_child_pair_at(node_id, position) {
                (Some(current_child), Some(expected_child)) => {
                    if current_child != expected_child {
                        node.replace_child(&expected_child, &current_child)?;
                    }
                }
                (Some(current_child), None) => {
                    node.remove_child(&current_child)?;
                    position = position.saturating_sub(1);
                }
                (None, Some(expected_child)) => {
                    node.append_child(&expected_child)?;
                }
                (None, None) => {
                    break;
                }
            }
            position += 1;
        }
        Ok(())
    }

    fn update_children_for_nodes(
        &self,
        affected_node_ids: &HashSet<usize>,
    ) -> Result<(), crate::Error> {
        for node_id in affected_node_ids.iter().copied() {
            self.update_children_for_node(node_id)?;
        }
        Ok(())
    }

    fn replace_nodes_that_require_update(
        &mut self,
    ) -> Result<HashSet<usize>, crate::Error> {
        let replace_node_ids = self
            .modified_nodes
            .iter()
            .copied()
            .filter_map(|node_id| match self.get_node(node_id) {
                Ok(editor_node) => match self.get_dom_node(node_id) {
                    Ok(dom_node) => {
                        if editor_node.requires_update(&dom_node) {
                            Some(Ok(node_id))
                        } else {
                            None
                        }
                    }
                    Err(err) => Some(Err(err)),
                },
                Err(err) => Some(Err(err)),
            })
            .collect::<Result<HashSet<_>, _>>()?;

        for node_id in replace_node_ids.iter().copied() {
            let editor_node = self.get_node(node_id)?;
            self.set_dom_node(node_id, editor_node.create_node()?);
        }

        Ok(replace_node_ids)
    }

    pub fn update_dom(&mut self) -> Result<(), crate::Error> {
        let created_node_ids = self.creating_missing_dom_nodes()?;
        let replaced_node_ids = self.replace_nodes_that_require_update()?;
        let affected_parent_ids = created_node_ids
            .into_iter()
            .chain(replaced_node_ids)
            .filter_map(|node_id| self.get_parent_id(node_id).ok().flatten())
            .collect::<HashSet<_>>()
            .union(&self.modified_nodes)
            .copied()
            .filter(|node_id| self.is_container_node(*node_id).unwrap_or(false))
            .collect::<HashSet<_>>();

        self.update_children_for_nodes(&affected_parent_ids)?;
        self.modified_nodes.clear();
        Ok(())
    }
}
