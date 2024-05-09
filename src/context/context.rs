use std::collections::HashMap;

use crate::node::Node;

#[derive(Debug, Clone)]
pub struct Context {
    objects: HashMap<String, Node>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            objects: HashMap::new(),
        }
    }

    pub fn lookup(&self, symbol: &String) -> Option<&Node> {
        self.objects.get(symbol)
    }

    pub fn insert(&mut self, name: &str, value: Node) {
        self.objects.insert(String::from(name), value);
    }
}
