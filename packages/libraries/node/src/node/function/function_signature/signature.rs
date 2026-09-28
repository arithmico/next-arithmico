use std::collections::HashSet;

use crate::NodeType;

use super::argument::Argument;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FunctionSignature {
    arguments: Vec<Argument>,
    return_type: HashSet<NodeType>,
}

impl FunctionSignature {
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

    pub fn get_return_type(&self) -> &HashSet<NodeType> {
        &self.return_type
    }

    pub fn arguments(&self) -> &[Argument] {
        &self.arguments
    }

    pub fn argument_names(&self) -> Vec<&str> {
        self.arguments()
            .iter()
            .map(|argument| argument.get_name())
            .collect()
    }

    pub fn has_argument(&self, name: &str) -> bool {
        self.arguments.iter().any(|arg| arg.get_name() == name)
    }
}
