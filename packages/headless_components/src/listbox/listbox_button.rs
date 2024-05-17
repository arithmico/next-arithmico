use web_sys::HtmlElement;
use yew::prelude::*;

use super::listbox_context::ListboxContext;
use super::ListboxValue;

#[derive(PartialEq, Properties)]
pub struct ListboxButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn ListboxButton<T: ListboxValue>(props: &ListboxButtonProps) -> Html {
    let arm_focus = use_state(|| false);
    let listbox_context = use_context::<ListboxContext<T>>().unwrap();
    let on_click = {
        let toggle_open = listbox_context.toggle_open.clone();
        Callback::from(move |_| toggle_open.emit(()))
    };
    let button_ref = use_node_ref();
    let is_open = listbox_context.is_open.clone();
    {
        let arm_focus = arm_focus.clone();
        use_effect_with(
            (is_open, button_ref.clone()),
            move |(is_open, button_ref)| {
                if *is_open && !*arm_focus {
                    arm_focus.set(true);
                } else if !*is_open && *arm_focus {
                    button_ref.cast::<HtmlElement>().unwrap().focus().unwrap();
                    arm_focus.set(false);
                }
            },
        );
    }
    let aria_controls = if is_open {
        Some(listbox_context.get_listbox_options_id())
    } else {
        None
    };

    html! {
        <button
            ref={button_ref}
            onclick={on_click}
            class={props.class.clone()}
            aria-haspopup={"listbox"}
            aria-expanded={is_open.to_string()}
            data-expanded={is_open.to_string()}
            aria-controls={aria_controls}
        >
            {props.children.clone()}
        </button>
    }
}
