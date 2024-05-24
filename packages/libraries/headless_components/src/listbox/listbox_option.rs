use web_sys::HtmlElement;
use yew::prelude::*;

use super::listbox_context::ListboxContext;
use super::listbox_options_context::ListboxOptionsContext;
use super::ListboxValue;

#[derive(PartialEq, Properties, Debug)]
pub struct ListboxOptionProps<T: ListboxValue> {
    pub value: T,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn ListboxOption<T: ListboxValue>(props: &ListboxOptionProps<T>) -> Html {
    let listbox_context = use_context::<ListboxContext<T>>().unwrap();
    let listbox_options_context =
        use_context::<ListboxOptionsContext<T>>().unwrap();
    let on_click = {
        let on_change = listbox_context.on_change.clone();
        let value = props.value.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            on_change.emit(value.clone());
        })
    };
    let li_ref = use_node_ref();
    {
        let li_ref = li_ref.clone();
        let focused_value = listbox_options_context.focused_value.clone();
        let option_value = props.value.clone();
        use_effect_with(
            (li_ref, focused_value, option_value),
            |(li_ref, selected_value, option_value)| {
                if selected_value == option_value {
                    li_ref.cast::<HtmlElement>().unwrap().focus().unwrap();
                }
            },
        );
    }
    let onkeypress = {
        let onkeypress = listbox_options_context.onkeypress.clone();
        let value = props.value.clone();
        Callback::from(move |event: KeyboardEvent| {
            onkeypress.clone().emit((event, value.clone()))
        })
    };
    let listbox_option_id = listbox_context.get_listbox_option_id(&props.value);
    let selected = listbox_context.value == props.value;

    html! {
        <li
            id={listbox_option_id}
            ref={li_ref}
            role={"option"}
            tabindex={"-1"}
            class={props.class.clone()}
            onclick={on_click}
            onkeydown={onkeypress}
            aria-selected={selected.to_string()}
            data-selected={selected.to_string()}
        >
            {props.children.clone()}
        </li>
    }
}
