use crate::core::{ForEachChild, Node};

use super::Negate;

impl ForEachChild for Negate {
    fn for_each_child<F: Fn(&Node) + Copy>(&self, f: F) {
        self.value.for_each_child(f);
    }

    fn for_each_child_mut<F: Fn(&mut Node) + Copy>(&mut self, f: F) {
        self.value.for_each_child_mut(f);
    }
}
