use js_sys::wasm_bindgen::JsCast;
use web_sys::{InputEvent, Node};

pub fn absolute_offset(
    node: &Node,
    offset: usize,
    container: &Node,
) -> Option<usize> {
    if node == container {
        return Some(offset);
    }

    // not a element node
    if node.node_type() != 1 {
        return None;
    }

    let mut current_total_offset: usize = 0;
    for child in node_children(&node) {
        if let Some(offset) = absolute_offset(&child, offset, container) {
            return Some(current_total_offset + offset);
        }
        current_total_offset += content_length(&child);
    }
    None
}

pub fn relative_offset(node: &Node, offset: usize) -> Option<(usize, Node)> {
    if content_length(node) < offset {
        return None;
    }

    match node.node_type() {
        // text node
        3 => Some((offset, node.clone())),
        // element node
        1 => {
            let mut current_offset = 0;
            for node in node_children(node) {
                if let Some(r) = relative_offset(&node, offset - current_offset)
                {
                    return Some(r);
                }
                current_offset += content_length(&node);
            }

            None
        }
        _ => None,
    }
}

pub fn content_length(node: &Node) -> usize {
    match node.node_type() {
        1 => node_children(&node)
            .iter()
            .map(|node| content_length(node))
            .reduce(|left, right| left + right)
            .unwrap_or(0),
        3 => node.text_content().expect("text content").len(),
        _ => 0,
    }
}

pub fn target_node(event: &InputEvent) -> Node {
    event
        .target()
        .expect("event target")
        .dyn_into::<Node>()
        .expect("node")
}

pub fn node_children(node: &Node) -> Vec<Node> {
    node.child_nodes()
        .values()
        .into_iter()
        .map(|node| node.expect("node").dyn_into::<Node>().expect("node"))
        .collect()
}
