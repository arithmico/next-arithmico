use node::{Division, Node};

use crate::core::ForEachChild;

impl ForEachChild for Division {
    fn for_each_child<F: Fn(&Node) + Copy>(&self, f: F) {
        self.dividend.for_each_child(f);
        self.divisor.for_each_child(f);
    }

    fn for_each_child_mut<F: Fn(&mut Node) + Copy>(&mut self, f: F) {
        self.dividend.for_each_child_mut(f);
        self.divisor.for_each_child_mut(f);
    }
}
