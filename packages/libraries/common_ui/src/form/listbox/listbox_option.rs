use leptos::prelude::*;
use leptos_dom::Transparent;

use super::ListboxOptionsDefinition;

#[component(transparent)]
pub fn ListboxOption<T: PartialEq + Clone + 'static>(
    value: T,
    children: ChildrenFn,
    #[prop(optional, into)] class: Option<ListboxOptionClass>,
) -> impl IntoView {
    ListboxOptionDefinition {
        value,
        children,
        class,
    }
}

#[derive(Clone)]
pub(super) struct ListboxOptionDefinition<T: PartialEq + Clone + 'static> {
    pub value: T,
    pub class: Option<ListboxOptionClass>,
    pub children: ChildrenFn,
}

impl<T: PartialEq + Clone + 'static> IntoView for ListboxOptionDefinition<T> {
    fn into_view(self) -> View {
        Transparent::new(self).into_view()
    }
}

#[derive(Clone)]
pub enum ListboxOptionClass {
    Value(String),
    Callback(Callback<bool, String>),
}

impl ListboxOptionClass {
    pub fn get_class(&self, selected: bool) -> String {
        match self {
            ListboxOptionClass::Value(class) => class.clone(),
            ListboxOptionClass::Callback(callback) => callback.call(selected),
        }
    }
}

impl From<&str> for ListboxOptionClass {
    fn from(value: &str) -> Self {
        ListboxOptionClass::Value(value.to_string())
    }
}

impl<R: ToString, T: Fn(bool) -> R + 'static> From<T> for ListboxOptionClass {
    fn from(value: T) -> Self {
        ListboxOptionClass::Callback(Callback::new(move |selected| {
            value(selected).to_string()
        }))
    }
}

impl<T: PartialEq + Clone + 'static> From<&ListboxOptionsDefinition>
    for Vec<ListboxOptionDefinition<T>>
{
    fn from(value: &ListboxOptionsDefinition) -> Self {
        value
            .options
            .iter()
            .map(|option| {
                option
                    .downcast_ref::<ListboxOptionDefinition<T>>()
                    .expect("ListboxOptionDefinition")
            })
            .cloned()
            .collect::<Vec<_>>()
    }
}
