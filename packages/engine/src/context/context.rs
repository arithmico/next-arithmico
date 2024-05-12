use std::{collections::HashMap, rc::Rc};

use crate::node::Node;

use super::{host_api::HostApi, settings::Settings, HostEndpoint};

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    stack: Vec<HashMap<String, Node>>,
    settings: Settings,
    host_api: Rc<HostApi>,
}

impl Context {
    pub fn new(host_api: Rc<HostApi>) -> Context {
        Context {
            stack: vec![HashMap::new()],
            settings: Settings::default(),
            host_api,
        }
    }

    pub fn lookup(&self, name: &String) -> Option<Node> {
        for stack_frame in self.stack.iter().rev() {
            if stack_frame.contains_key(name) {
                return stack_frame
                    .get(name)
                    .and_then(|node| Some(node.clone()));
            }
        }
        if let Some(endpoint) = self.host_api.endpoint(name) {
            return match endpoint {
                HostEndpoint::Function(_) => {
                    Some(Node::HostApiFunctionEndpoint { name: name.clone() })
                }
                HostEndpoint::Constant(f) => Some(f(self)),
            };
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
