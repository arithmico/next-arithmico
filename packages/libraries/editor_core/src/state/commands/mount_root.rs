use web_sys::Node;

use crate::state::EditorState;

impl EditorState {
    pub fn mount_to_root(&mut self, dom_node: Node) {
        let previous_root = self.get_dom_node(self.get_root_id());
        self.set_dom_node(self.get_root_id(), dom_node);
        if previous_root.is_some() {
            self.modified_nodes.insert(self.get_root_id());
        }
        self.update_dom();
        self.write_selection_to_dom();
    }
}
