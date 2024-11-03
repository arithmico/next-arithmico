use web_sys::Node;

use crate::core::state::EditorState;

impl EditorState {
    pub fn mount_to_root(&mut self, dom_node: Node) {
        self.set_dom_node(self.get_root_id(), dom_node);
        self.update_dom();
    }
}
