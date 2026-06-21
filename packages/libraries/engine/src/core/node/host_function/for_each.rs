use node::{HostFunction, Node};

use crate::core::ForEachChild;

impl ForEachChild for HostFunction {
    fn for_each_child<F: Fn(&Node) + Copy>(&self, _f: F) {}

    fn for_each_child_mut<F: Fn(&mut Node) + Copy>(&mut self, _f: F) {}
}
