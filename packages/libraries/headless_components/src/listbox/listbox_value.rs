use std::fmt::Debug;
use std::hash::Hash;

pub trait ListboxValue: Debug + PartialEq + Hash + Clone + 'static {}
impl<T: Debug + PartialEq + Hash + Clone + 'static> ListboxValue for T {}
