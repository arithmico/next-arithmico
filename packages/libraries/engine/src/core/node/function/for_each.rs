use crate::core::{ForEachChild, Node};

use super::Function;

impl ForEachChild for Function {
    fn for_each_child<F: Fn(&Node) + Copy>(&self, f: F) {
        self.expression.for_each_child(f);
    }

    fn for_each_child_mut<F: Fn(&mut Node) + Copy>(&mut self, f: F) {
        self.expression.for_each_child_mut(f);
    }
}
