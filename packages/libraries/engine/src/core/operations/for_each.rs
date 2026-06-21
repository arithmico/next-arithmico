use node::Node;

#[allow(dead_code)]
pub trait ForEachChild {
    fn for_each_child<F: Fn(&Node) + Copy>(&self, f: F);
    fn for_each_child_mut<F: Fn(&mut Node) + Copy>(&mut self, f: F);
}
