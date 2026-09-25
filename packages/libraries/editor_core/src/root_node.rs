use std::{any::Any, sync::Arc};

use leptos::prelude::document;
use web_sys::wasm_bindgen::JsCast;

use crate::EditorContainerNode;

#[derive(Clone, Debug)]
pub struct RootNode;

impl Default for RootNode {
    fn default() -> Self {
        Self
    }
}

impl EditorContainerNode for RootNode {
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

    fn as_any(&self) -> Arc<&dyn Any> {
        Arc::new(self)
    }

    fn arc_clone(&self) -> Arc<dyn EditorContainerNode> {
        Arc::new(self.clone())
    }

    fn into_editor_node(self) -> crate::EditorNode {
        crate::EditorNode::Container(Arc::new(self))
    }

    fn delete_if_empty(&self) -> bool {
        false
    }
}
