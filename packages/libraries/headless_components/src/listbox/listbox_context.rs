use std::hash::{DefaultHasher, Hasher};

use yew::prelude::*;

use super::ListboxValue;

#[derive(PartialEq, Debug, Clone)]
pub struct ListboxContext<T: ListboxValue> {
    pub value: T,
    pub is_open: bool,
    pub on_change: Callback<T>,
    pub toggle_open: Callback<()>,
    pub listbox_id: String,
}

impl<T: ListboxValue> ListboxContext<T> {
    pub fn get_listbox_options_id(&self) -> String {
        format!("{}-options", self.listbox_id)
    }

    fn get_listbox_option_hash(&self, value: &T) -> String {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    pub fn get_listbox_option_id(&self, value: &T) -> String {
        format!(
            "{}-{}",
            self.listbox_id,
            self.get_listbox_option_hash(value)
        )
    }
}
