use web_sys::Node;

use crate::state::EditorState;

impl EditorState {
    pub fn get_parent_dom_node(
        &self,
        node_id: usize,
    ) -> Result<Option<Node>, crate::Error> {
        match self.get_parent_id(node_id)? {
            Some(parent_id) => Ok(Some(self.get_dom_node(parent_id)?)),
            None => Ok(None),
        }
    }
}
