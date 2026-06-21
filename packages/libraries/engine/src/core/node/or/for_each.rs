use node::{Node, Or};

use crate::core::ForEachChild;

impl ForEachChild for Or {
    fn for_each_child<F: Fn(&Node) + Copy>(&self, f: F) {
        self.elements.iter().for_each(|node| node.for_each_child(f));
    }

    fn for_each_child_mut<F: Fn(&mut Node) + Copy>(&mut self, f: F) {
        self.elements
            .iter_mut()
            .for_each(|node| node.for_each_child_mut(f));
    }
}
