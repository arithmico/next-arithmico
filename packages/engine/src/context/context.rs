use std::collections::HashMap;

use crate::node::Node;

use super::settings::Settings;

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    stack: Vec<HashMap<String, Node>>,
    settings: Settings,
}

impl Context {
    pub fn new() -> Self {
        Context {
            stack: vec![HashMap::new()],
            settings: Settings::default(),
        }
    }

    pub fn lookup(&self, name: &String) -> Option<&Node> {
        for stack_frame in self.stack.iter() {
            if stack_frame.contains_key(name) {
                return stack_frame.get(name);
            }
        }
        None
    }

    pub fn insert(&mut self, name: &str, value: Node) {
        if self.stack.is_empty() {
            self.stack.push(HashMap::new())
        }
        self.stack
            .last_mut()
            .unwrap()
            .insert(String::from(name), value);
    }

    pub fn push_frame(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn get_decimal_places(&self) -> u8 {
        self.settings.get_decimal_places()
    }
}
