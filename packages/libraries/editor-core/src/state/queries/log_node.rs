use leptos_dom::log;

use crate::{EditorNode, EditorState};

impl EditorState {
    // TODO: consider Result instead of Option
    fn get_log_lines_for_node(&self, node_id: usize) -> Option<Vec<String>> {
        let mut lines = Vec::new();
        let node = self.get_node(node_id).ok()?;
        match node {
            EditorNode::Container(_) => {
                lines.push(format!("Container ({})", node_id));
                let children = self
                    .get_children_ids(node_id)
                    .into_iter()
                    .filter_map(|node_id| self.get_log_lines_for_node(node_id))
                    .flatten()
                    .map(|line| format!("  {}", line));

                for line in children {
                    lines.push(line);
                }
            }
            EditorNode::Leaf(node) => {
                lines.push(format!(
                    "Leaf ({}): \"{}\"",
                    node_id,
                    node.serialize()
                ));
            }
        }

        Some(lines)
    }

    pub fn log_node(&self, node_id: usize) -> Option<String> {
        Some(self.get_log_lines_for_node(node_id)?.join("\n"))
    }

    pub fn log_state(&self) {
        log!("{}", self.log_node(self.get_root_id()).unwrap_or_default());
    }
}
