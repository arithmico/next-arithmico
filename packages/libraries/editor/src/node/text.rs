use std::{any::Any, sync::Arc};

use editor_core::{EditorLeafNode, EditorNode};
use leptos::prelude::document;
use unicode_segmentation::UnicodeSegmentation;
use web_sys::{Text, wasm_bindgen::JsCast};

#[derive(Debug, Clone)]
pub struct TextNode {
    content: String,
}

impl Default for TextNode {
    fn default() -> Self {
        Self::new_with_content("")
    }
}

impl TextNode {
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
    fn create_node(&self) -> Result<web_sys::Node, editor_core::Error> {
        let text = document().create_text_node(&self.content);
        text.dyn_into()
            .map_err(|_| editor_core::Error::NodeCastFailed)
    }

    fn requires_update(&self, dom_node: &web_sys::Node) -> bool {
        match dom_node.clone().dyn_into::<Text>() {
            Ok(text_node) => text_node
                .text_content()
                .map(|content| content != self.content)
                .unwrap_or(true),
            _ => true,
        }
    }

    fn as_any(&self) -> Arc<&dyn Any> {
        Arc::new(self)
    }

    fn into_editor_node(self) -> editor_core::EditorNode {
        EditorNode::Leaf(Arc::new(self))
    }

    fn arc_clone(&self) -> Arc<dyn EditorLeafNode> {
        Arc::new(self.clone())
    }

    fn length(&self) -> usize {
        self.content.len()
    }

    fn slice(&self, start: usize, end: usize) -> EditorNode {
        let new_content = &self.content[start..end];
        TextNode::new_with_content(new_content).into_editor_node()
    }

    fn get_whitespaces(&self) -> Vec<(usize, usize)> {
        self.content
            .grapheme_indices(true)
            .filter_map(
                |(index, grapheme)| {
                    if grapheme == " " { Some(index) } else { None }
                },
            )
            .fold(Vec::<(usize, usize)>::new(), |mut whitespaces, pos| {
                if let Some(last) = whitespaces.last_mut() {
                    if last.1 + 1 == pos {
                        last.1 = pos;
                    } else {
                        whitespaces.push((pos, pos));
                    }
                } else {
                    whitespaces.push((pos, pos));
                };

                whitespaces
            })
    }

    fn serialize(&self) -> String {
        self.content.clone()
    }
}
