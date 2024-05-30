use crate::core::node::*;

#[derive(PartialEq, Debug, Clone)]
pub struct HostFunction {
    pub name: String,
}

impl HostFunction {
    pub fn new<T: Into<String>>(name: T) -> HostFunction {
        HostFunction { name: name.into() }
    }
}

impl From<HostFunction> for Node {
    fn from(value: HostFunction) -> Node {
        Node::HostApiFunctionEndpoint(value)
    }
}

impl BaseNode for HostFunction {}
