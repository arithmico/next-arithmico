use yew::prelude::*;

use super::ListboxValue;

#[derive(Clone, PartialEq)]
pub struct ListboxOptionsContext<T: ListboxValue> {
    pub onkeypress: Callback<(KeyboardEvent, T)>,
    pub focused_value: T,
}
