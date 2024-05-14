use std::cmp::min;

use yew::prelude::*;

use super::{
    listbox_context::ListboxContext,
    listbox_options_context::ListboxOptionsContext, ListboxOption,
    ListboxValue,
};

#[derive(PartialEq, Properties)]
pub struct ListboxOptionsProps<T: ListboxValue> {
    pub children: ChildrenWithProps<ListboxOption<T>>,
    #[prop_or_default]
    pub class: Option<Classes>,
}

#[function_component]
pub fn ListboxOptions<T: ListboxValue>(props: &ListboxOptionsProps<T>) -> Html {
    let listbox_context = use_context::<ListboxContext<T>>().unwrap();
    let values: Vec<_> = props.children.iter().map(|item| item.props).collect();
    let focused_value = {
        let value = listbox_context.value.clone();
        use_state(move || value)
    };

    let onkeypress: Callback<(KeyboardEvent, T)> = {
        let current_value = listbox_context.value.clone();
        let toggle_open = listbox_context.toggle_open.clone();
        let values: Vec<T> =
            values.iter().map(|props| props.value.clone()).collect();
        let onchange = listbox_context.on_change.clone();
        let focused_value = focused_value.clone();
        Callback::from(move |(event, value): (KeyboardEvent, T)| {
            let key = event.key();
            let value_index =
                values.iter().position(|element| *element == value).unwrap();
            match key.as_str() {
                "ArrowDown" => {
                    let new_index = min(values.len() - 1, value_index + 1);
                    focused_value.set(values.get(new_index).unwrap().clone());
                }
                "ArrowUp" => {
                    if value_index >= 1 {
                        focused_value
                            .set(values.get(value_index - 1).unwrap().clone());
                    }
                }
                " " => {
                    onchange.emit(value);
                }
                "Enter" => {
                    onchange.emit(value);
                    event.prevent_default();
                }
                "Escape" => {
                    focused_value.set(current_value.clone());
                    toggle_open.emit(());
                }
                "Tab" => {
                    event.prevent_default();
                }
                _ => (),
            }
        })
    };

    let context = ListboxOptionsContext {
        onkeypress,
        focused_value: (*focused_value).clone(),
    };

    let listbox_options_id = listbox_context.get_listbox_options_id();

    html! {
        if listbox_context.is_open {
            <ContextProvider<ListboxOptionsContext<T>> context={context}>
                <ul
                    id={listbox_options_id}
                    tabindex={0}
                    class={props.class.clone()}
                    >
                    {for props.children.iter()}
                </ul>
            </ContextProvider<ListboxOptionsContext<T>>>
        }
    }
}
