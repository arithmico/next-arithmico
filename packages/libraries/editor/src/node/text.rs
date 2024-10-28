use std::any::Any;

use leptos::document;
use web_sys::{wasm_bindgen::JsCast, Text};

use crate::core::{EditorLeafNode, EditorNode};

#[derive(Debug, Clone)]
pub struct TextNode {
    content: String,
}

impl TextNode {
    pub fn new() -> Self {
        Self::new_with_content("")
    }

    pub fn new_with_content<T: ToString>(content: T) -> Self {
        Self {
            content: content.to_string(),
        }
    }

    pub fn insert_text(
        &self,
        text: String,
        start_index: usize,
        end_index: usize,
    ) -> Self {
        let mut new_content = self.content.clone();
        new_content.replace_range(start_index..end_index, &text);
        Self {
            content: new_content,
        }
    }

    pub fn append(&self, other: &Self) -> Self {
        Self {
            content: format!("{}{}", self.content, other.content),
        }
    }
}

impl EditorLeafNode for TextNode {
    fn create_node(&self) -> web_sys::Node {
        let text = document().create_text_node(&self.content);
        return text.dyn_into().expect("node");
    }

    fn requires_update(&self, dom_node: &web_sys::Node) -> bool {
        if let Ok(text_node) = dom_node.clone().dyn_into::<Text>() {
            let requires_update = text_node
                .text_content()
                .and_then(|content| Some(content != self.content))
                .unwrap_or(true);
            requires_update
        } else {
            true
        }
    }

    fn as_any(&self) -> Box<&dyn Any> {
        Box::new(self)
    }

    fn boxed_clone(&self) -> Box<dyn EditorNode> {
        Box::new(self.clone())
    }
}
