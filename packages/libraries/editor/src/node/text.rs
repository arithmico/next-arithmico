use std::{any::Any, sync::Arc};

use editor_core::{EditorLeafNode, EditorNode};
use leptos::prelude::document;
use unicode_segmentation::UnicodeSegmentation;
use web_sys::{Text, wasm_bindgen::JsCast};

#[derive(Debug, Clone)]
pub struct TextNode {
    content: String,
    chars: usize,
}

impl Default for TextNode {
    fn default() -> Self {
        Self::new_with_content("")
    }
}

impl TextNode {
    pub fn new_with_content<T: ToString>(content: T) -> Self {
        let content = content.to_string();

        Self {
            chars: content.chars().count(),
            content,
        }
    }

    pub fn insert_text(
        &self,
        text: String,
        start_index: usize,
        end_index: usize,
    ) -> Self {
        let mut new_content = String::with_capacity(
            (self.content.len() + text.len())
                .saturating_sub(end_index.abs_diff(start_index)),
        );
        let mut inserted = false;
        for (i, c) in self.content.char_indices() {
            if i < start_index || i >= end_index {
                new_content.push(c);
            } else if !inserted {
                new_content.push_str(&text);
                inserted = true;
            }
        }
        Self::new_with_content(new_content)
    }

    pub fn append(&self, other: &Self) -> Self {
        Self::new_with_content(format!("{}{}", self.content, other.content))
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
        // TODO: consider caching
        self.chars
    }

    fn slice(&self, start: usize, end: usize) -> EditorNode {
        let new_content = self.content.graphemes(true).collect::<Vec<_>>()
            [start..end]
            .join("");

        TextNode::new_with_content(new_content).into_editor_node()
    }

    fn get_whitespaces(&self) -> Vec<(usize, usize)> {
        self.content
            .char_indices()
            .filter_map(
                |(index, char)| {
                    if char == ' ' { Some(index) } else { None }
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
