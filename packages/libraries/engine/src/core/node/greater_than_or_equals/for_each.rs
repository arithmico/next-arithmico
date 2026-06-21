use node::{GreaterThanOrEquals, Node};

use crate::core::ForEachChild;

impl ForEachChild for GreaterThanOrEquals {
    fn for_each_child<F: Fn(&Node) + Copy>(&self, f: F) {
        self.left.for_each_child(f);
        self.right.for_each_child(f);
    }

    fn for_each_child_mut<F: Fn(&mut Node) + Copy>(&mut self, f: F) {
        self.left.for_each_child_mut(f);
        self.right.for_each_child_mut(f);
    }
}
