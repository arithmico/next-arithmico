use std::collections::HashMap;

use ast::Node;

pub type Stackframe = HashMap<String, Node>;

#[derive(Debug, Clone, PartialEq)]
pub struct Stack {
    frames: Vec<Stackframe>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            frames: vec![HashMap::new()],
        }
    }

    pub fn get_frames(&self) -> &Vec<Stackframe> {
        &self.frames
    }

    pub fn add_frame(&mut self) {
        self.frames.push(HashMap::new());
    }

    pub fn insert(&mut self, name: &str, node: Node) {
        if self.frames.is_empty() {
            self.frames.push(HashMap::new());
        }
        self.frames.last_mut().unwrap().insert(name.into(), node);
    }

    pub fn lookup(&self, name: &str) -> Option<Node> {
        for frame in &self.frames {
            let node_option = frame.get(name);
            if let Some(node) = node_option {
                return Some(node.clone());
            }
        }
        None
    }

    pub fn entries(&self) -> HashMap<String, Node> {
        let mut result = HashMap::new();
        for frame in self.frames.iter() {
            for (key, value) in frame {
                result.insert(key.clone(), value.clone());
            }
        }
        result
    }
}
