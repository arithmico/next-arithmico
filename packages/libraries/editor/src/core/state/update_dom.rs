use std::{cmp::Ordering, collections::HashSet};

use web_sys::{wasm_bindgen::JsCast, Node};

use super::EditorState;

fn update_child_for_node(
    node: &Node,
    old_child: Option<&Node>,
    new_child: Option<&Node>,
) {
    match (old_child, new_child) {
        (Some(old_child), Some(new_child)) => {
            if old_child != new_child {
                node.replace_child(new_child, old_child)
                    .expect("replace child");
            }
        }
        (None, Some(new_child)) => {
            node.append_child(new_child).expect("append_child");
        }
        (Some(old_child), None) => {
            node.remove_child(old_child).expect("remove child");
        }
        _ => unreachable!(),
    }
}

impl EditorState {
    fn read_current_children_from_dom(
        &self,
        node_id: usize,
    ) -> Option<Vec<Node>> {
        self.get_dom_node(node_id)
            .map(|node| {
                node.child_nodes()
                    .values()
                    .into_iter()
                    .map(|child| child?.dyn_into::<Node>())
                    .collect::<Result<Vec<Node>, _>>()
                    .ok()
            })
            .flatten()
    }

    #[allow(dead_code)]
    fn sort_modified_nodes(&self) -> Vec<usize> {
        let mut as_vec =
            self.modified_nodes.iter().copied().collect::<Vec<_>>();
        as_vec.sort_by(|left, right| {
            if left == right {
                return Ordering::Equal;
            }
            if self.is_parent_of(*left, *right) {
                return Ordering::Less;
            }
            if self.is_parent_of(*right, *left) {
                return Ordering::Greater;
            }
            Ordering::Equal
        });
        as_vec
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

    fn update_children_for_node(&self, node_id: usize) {
        let node = self.get_dom_node(node_id).expect("parent dom node");

        let old_children = self
            .read_current_children_from_dom(node_id)
            .expect("old children");

        let new_children =
            self.get_children_dom_nodes(node_id).expect("new children");

        for i in 0..old_children.len().max(new_children.len()) {
            let old_child = old_children.get(i);
            let new_child = new_children.get(i);
            update_child_for_node(node, old_child, new_child);
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
                editor_node.requires_update(dom_node)
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
            .chain(replaced_node_ids.into_iter())
            .filter_map(|node_id| self.get_parent_id(node_id))
            .collect::<HashSet<_>>();

        self.update_children_for_nodes(&affected_parent_ids);
        self.modified_nodes.clear();
    }
}
