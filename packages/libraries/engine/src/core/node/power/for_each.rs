use node::{Node, Power};

use crate::core::ForEachChild;

impl ForEachChild for Power {
    fn for_each_child<F: Fn(&Node) + Copy>(&self, f: F) {
        self.base.for_each_child(f);
        self.exponent.for_each_child(f);
    }

    fn for_each_child_mut<F: Fn(&mut Node) + Copy>(&mut self, f: F) {
        self.base.for_each_child_mut(f);
        self.exponent.for_each_child_mut(f);
    }
}
