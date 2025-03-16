use crate::core::{ForEachChild, Node};

use super::HostFunction;

impl ForEachChild for HostFunction {
    fn for_each_child<F: Fn(&Node) + Copy>(&self, _f: F) {}

    fn for_each_child_mut<F: Fn(&mut Node) + Copy>(&mut self, _f: F) {}
}
