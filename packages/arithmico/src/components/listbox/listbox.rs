use headless_components::listbox::{
    Listbox as HeadlessListbox, ListboxButton as HeadlessListboxButton,
    ListboxButtonProps, ListboxOption as HeadlessListboxOption,
    ListboxOptionProps, ListboxOptionProps as HeadlessListboxOptionProps,
    ListboxOptions as HeadlessListboxOptions, ListboxProps, ListboxValue,
};
use yew::{prelude::*, virtual_dom::VChild};

#[function_component]
pub fn Listbox<T: ListboxValue>(props: &ListboxProps<T>) -> Html {
    html! {
        <HeadlessListbox<T>
            on_change={props.on_change.clone()}
            value={props.value.clone()}
            class={classes!(
                "relative",
                "flex",
                "flex-col",
                "w-36",
                props.class.clone()
            )}
        >
            {props.children.clone()}
        </HeadlessListbox<T>>
    }
}

#[function_component]
pub fn ListboxButton<T: ListboxValue>(props: &ListboxButtonProps) -> Html {
    html! {
        <HeadlessListboxButton<T>
            class={classes!(
                "bg-neutral-300",
                "px-2",
                "py-1",
                "rounded-sm",
                "w-full",
                "items-center",
                "flex",
                "group",
                props.class.clone()
            )}
        >
            {props.children.clone()}
        </HeadlessListboxButton<T>>
    }
}

#[derive(PartialEq, Properties, Debug)]
pub struct ListboxOptionsProps<T: ListboxValue> {
    pub children: ChildrenWithProps<ListboxOption<T>>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn ListboxOptions<T: ListboxValue>(props: &ListboxOptionsProps<T>) -> Html {
    let transformed_children = props.children.iter().map(
        |child| -> VChild<HeadlessListboxOption<T>> {
            VChild::new(
                HeadlessListboxOptionProps {
                    children: child.props.children.clone(),
                    value: child.props.value.clone(),
                    class: classes!(
                        "px-2",
                        "py-1",
                        "rounded-sm",
                        "w-full",
                        "text-left",
                        "outline-black",
                        "focus:outline-2",
                        "focus-visible:outline-2",
                        "hover:bg-neutral-400/50",
                        child.props.class.clone()
                    ),
                },
                None,
            )
        },
    );

    html! {
        <HeadlessListboxOptions<T>
            class={classes!(
                "absolute",
                "bg-neutral-300",
                "mt-2",
                "w-full",
                "rounded-sm",
                "outline-black",
                "focus:outline-2",
                "focus-visible:outline-2",
                props.class.clone()
            )}
        >
            {for transformed_children}
        </HeadlessListboxOptions<T>>
    }
}

#[function_component]
pub fn ListboxOption<T: ListboxValue>(_props: &ListboxOptionProps<T>) -> Html {
    html! {}
}
