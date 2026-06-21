use node::{FunctionCall, Node};

use crate::core::ForEachChild;

impl ForEachChild for FunctionCall {
    fn for_each_child<F: Fn(&Node) + Copy>(&self, f: F) {
        self.target.for_each_child(f);
        self.arguments
            .iter()
            .for_each(|argument| argument.for_each_child(f));
    }

    fn for_each_child_mut<F: Fn(&mut Node) + Copy>(&mut self, f: F) {
        self.target.for_each_child_mut(f);
        self.arguments
            .iter_mut()
            .for_each(|argument| argument.for_each_child_mut(f));
    }
}
