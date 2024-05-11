use std::collections::HashMap;

use crate::node::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct DecimalPlaces(u8);

impl From<u8> for DecimalPlaces {
    fn from(value: u8) -> Self {
        if value > 15 {
            panic!("invalid decimal places");
        }
        DecimalPlaces(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    stack: Vec<HashMap<String, Node>>,
    decimal_places: DecimalPlaces,
}

impl Context {
    pub fn new() -> Self {
        Context {
            stack: vec![HashMap::new()],
            decimal_places: 5.into(),
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
        self.decimal_places.0
    }
}
