use leptos_dom::log;

use crate::{EditorNode, EditorState};

impl EditorState {
    fn get_log_lines_for_node(&self, node_id: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let node = self.get_node(node_id).expect("node");
        match node {
            EditorNode::Container(_) => {
                lines.push(format!("Container ({})", node_id));
                let children = self
                    .get_children_ids(node_id)
                    .into_iter()
                    .flat_map(|node_id| self.get_log_lines_for_node(node_id))
                    .map(|line| format!("  {}", line));

                for line in children {
                    lines.push(line);
                }
            }
            EditorNode::Leaf(node) => {
                lines.push(format!(
                    "Leaf ({}): \"{}\"",
                    node_id,
                    node.serialize().expect("serialized leaf node")
                ));
            }
        }

        lines
    }

    pub fn log_node(&self, node_id: usize) -> String {
        self.get_log_lines_for_node(node_id).join("\n")
    }

    pub fn log_state(&self) {
        log!("{}", self.log_node(self.get_root_id()));
    }
}
