use std::{any::Any, sync::Arc};

use editor_core::{EditorLeafNode, EditorNode};
use leptos::prelude::document;
use web_sys::{Text, wasm_bindgen::JsCast};

#[derive(Debug, Clone)]
pub struct TextNode {
    content: String,
    /// UTF16 length!
    length: usize,
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
            length: content.encode_utf16().count(),
            content,
        }
    }

    pub fn insert_text(
        &self,
        text: String,
        start_index: usize,
        end_index: usize,
    ) -> Self {
        let mut new_content = self.content.encode_utf16().collect::<Vec<_>>();
        let new_content = new_content
            .splice(start_index..end_index, text.encode_utf16())
            .collect::<Vec<_>>();
        Self::new_with_content(
            String::from_utf16(&new_content).unwrap_or_default(),
        )
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
        self.length
    }

    fn previous_character_boundary(
        &self,
        pos: usize,
    ) -> Option<(usize, usize)> {
        if pos == 0 {
            return None;
        }

        let u16_units = self.content.encode_utf16().collect::<Vec<_>>();
        let len = u16_units.len();

        if pos > len {
            return None;
        }

        let char_len = if pos >= 2
            && (0xDC00..=0xDFFF).contains(&u16_units[pos - 1])
            && (0xD800..=0xDBFF).contains(&u16_units[pos - 2])
        {
            2
        } else {
            1
        };

        Some((pos - char_len, pos))
    }

    fn slice(&self, mut start: usize, mut end: usize) -> EditorNode {
        let content = self.content.encode_utf16().collect::<Vec<_>>();
        start = start.min(content.len().saturating_sub(1));
        end = end.min(content.len());

        if content.is_empty() {
            return TextNode::new_with_content("").into_editor_node();
        }

        if (0xDC00..=0xDFFF).contains(&content[start]) {
            start += 1;
        }

        if (0xD800..=0xDBFF).contains(&content[end.saturating_sub(1)]) {
            end = end.saturating_sub(1);
        }

        if start >= end {
            TextNode::new_with_content("").into_editor_node()
        } else {
            let new_content = String::from_utf16_lossy(&content[start..end]);
            TextNode::new_with_content(new_content).into_editor_node()
        }
    }

    fn get_whitespaces(&self) -> Vec<(usize, usize)> {
        self.content
            .encode_utf16()
            .enumerate()
            .filter_map(|(index, code)| {
                let s = String::from_utf16_lossy(&[code]);
                s.chars().next().and_then(|s| {
                    if s.is_whitespace() { Some(index) } else { None }
                })
            })
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
