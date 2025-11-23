use std::collections::HashSet;

use crate::core::NodeType;

use super::argument::Argument;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    arguments: Vec<Argument>,
    return_type: HashSet<NodeType>,
}

impl FunctionSignature {
    pub fn new() -> Self {
        Self {
            arguments: Vec::new(),
            return_type: HashSet::new(),
        }
    }

    pub fn argument<T: ToString>(
        mut self,
        name: T,
        argument_configurator: fn(Argument) -> Argument,
    ) -> Self {
        self.add_argument(name, argument_configurator);
        self
    }

    pub fn add_argument<T: ToString>(
        &mut self,
        name: T,
        argument_configurator: fn(Argument) -> Argument,
    ) {
        let name = name.to_string();
        if self
            .arguments
            .iter()
            .find(|argument| argument.get_name() == name)
            .is_some()
        {
            panic!("duplicate parameter name");
        }

        self.arguments
            .push(argument_configurator(Argument::new(name)));
    }

    pub fn add_return_type(mut self, return_type: NodeType) -> Self {
        self.return_type.insert(return_type);
        self
    }

    pub fn get_return_type(&self) -> Option<&NodeType> {
        if self.return_type.len() == 1 {
            self.return_type.iter().next()
        } else {
            None
        }
    }

    pub fn arguments(&self) -> &[Argument] {
        &self.arguments
    }

    pub fn argument_names(&self) -> Vec<String> {
        self.arguments()
            .iter()
            .map(|argument| argument.get_name())
            .collect()
    }
}
