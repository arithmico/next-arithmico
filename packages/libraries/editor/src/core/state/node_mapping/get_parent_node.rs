use web_sys::Node;

use crate::core::EditorNode;

use super::NodeMapping;

#[allow(dead_code)]
impl NodeMapping {
    pub fn parent_id(&self, node_id: usize) -> Option<usize> {
        self.parent
            .get(&node_id)
            .map(|parent| parent.as_ref().copied())
            .flatten()
    }

    pub fn get_parent_node(
        &self,
        node_id: usize,
    ) -> Option<&Box<dyn EditorNode>> {
        self.parent_id(node_id)
            .map(|parent_id| self.get_node(parent_id))
            .flatten()
    }

    pub fn get_parent_dom_node(&self, node_id: usize) -> Option<&Node> {
        self.parent_id(node_id)
            .map(|parent_id| self.get_dom_node(parent_id))
            .flatten()
    }

    pub fn is_parent_of(&self, parent_id: usize, child_id: usize) -> bool {
        let mut current_node_id = self.parent_id(child_id);
        while let Some(node_id) = current_node_id {
            if node_id == parent_id {
                return true;
            }
            current_node_id = self.parent_id(node_id);
        }
        false
    }
}
