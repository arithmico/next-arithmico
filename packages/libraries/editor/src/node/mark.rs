use editor_core::EditorContainerNode;
use leptos::document;
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
        let class = "bg-red-300 rounded-sm";
        node.set_class_name(&class);
        node.dyn_into::<Node>().expect("node")
    }

    fn requires_update(&self, _dom_node: &web_sys::Node) -> bool {
        false
    }

    fn as_any(&self) -> Box<&dyn std::any::Any> {
        Box::new(self)
    }

    fn boxed_clone(&self) -> Box<dyn EditorContainerNode> {
        Box::new(self.clone())
    }

    fn into_editor_node(self) -> editor_core::EditorNode {
        editor_core::EditorNode::Container(Box::new(self))
    }

    fn delete_if_empty(self) -> bool {
        true
    }
}
