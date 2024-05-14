use uuid::Uuid;
use yew::prelude::*;

use super::listbox_context::ListboxContext;
use super::ListboxValue;

#[derive(PartialEq, Properties)]
pub struct ListboxProps<T: ListboxValue> {
    pub value: T,
    #[prop_or_default]
    pub class: Option<Classes>,
    pub children: Children,
    pub on_change: Callback<T>,
}

#[function_component]
pub fn Listbox<T: ListboxValue>(props: &ListboxProps<T>) -> Html {
    let listbox_id = use_state(|| Uuid::new_v4().hyphenated().to_string());
    let is_open = use_state(|| false);
    let toggle_open = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };
    let onchange = {
        let onchange = props.on_change.clone();
        let is_open = is_open.clone();
        Callback::from(move |value| {
            is_open.set(false);
            onchange.emit(value);
        })
    };

    let context = ListboxContext {
        is_open: *is_open,
        value: props.value.clone(),
        on_change: onchange,
        toggle_open,
        listbox_id: (*listbox_id).clone(),
    };
    let active_activedescendant = context.get_listbox_option_id(&context.value);

    html! {
        <ContextProvider<ListboxContext<T>> context={context}>
            <div
                id={listbox_id.to_string()}
                class={props.class.clone()}
                role={"listbox"}
                aria-activedescendant={active_activedescendant}
            >
                {props.children.clone()}
            </div>
        </ContextProvider<ListboxContext<T>>>
    }
}
