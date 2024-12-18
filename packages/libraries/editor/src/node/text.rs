use std::any::Any;

use editor_core::{EditorLeafNode, EditorNode};
use leptos_dom::document;
use unicode_segmentation::UnicodeSegmentation;
use web_sys::{wasm_bindgen::JsCast, Text};

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

    fn into_editor_node(self) -> editor_core::EditorNode {
        EditorNode::Leaf(Box::new(self))
    }

    fn boxed_clone(&self) -> Box<dyn EditorLeafNode> {
        Box::new(self.clone())
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
                    if grapheme == " " {
                        Some(index)
                    } else {
                        None
                    }
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

    fn serialize(&self) -> Option<String> {
        Some(self.content.clone())
    }
}
