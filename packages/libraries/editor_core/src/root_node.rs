use std::any::Any;

use leptos::document;
use web_sys::wasm_bindgen::JsCast;

use crate::EditorNode;

#[derive(Clone)]
pub struct RootNode;

impl RootNode {
    pub fn new() -> Self {
        Self
    }
}

impl EditorNode for RootNode {
    fn create_node(&self) -> web_sys::Node {
        document()
            .create_element("div")
            .expect("root element")
            .dyn_into()
            .expect("root node")
    }

    fn requires_update(&self, _dom_node: &web_sys::Node) -> bool {
        false
    }

    fn as_any(&self) -> Box<&dyn Any> {
        Box::new(self)
    }

    fn supports_children(&self) -> bool {
        true
    }

    fn boxed_clone(&self) -> Box<dyn EditorNode> {
        Box::new(self.clone())
    }
}
