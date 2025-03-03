use std::sync::Arc;

use editor_core::EditorContainerNode;
use leptos::prelude::document;
use web_sys::{wasm_bindgen::JsCast, Node};

#[derive(Debug, Clone)]
pub struct MarkNode;

impl MarkNode {
    pub fn new() -> Self {
        Self
    }
}

impl EditorContainerNode for MarkNode {
    fn create_node(&self) -> web_sys::Node {
        let node = document().create_element("span").expect("element");
        let class = "editor-error-mark";
        node.set_class_name(&class);
        node.dyn_into::<Node>().expect("node")
    }

    fn requires_update(&self, _dom_node: &web_sys::Node) -> bool {
        false
    }

    fn as_any(&self) -> Arc<&dyn std::any::Any> {
        Arc::new(self)
    }

    fn arc_clone(&self) -> Arc<dyn EditorContainerNode> {
        Arc::new(self.clone())
    }

    fn into_editor_node(self) -> editor_core::EditorNode {
        editor_core::EditorNode::Container(Arc::new(self))
    }

    fn delete_if_empty(&self) -> bool {
        true
    }
}
