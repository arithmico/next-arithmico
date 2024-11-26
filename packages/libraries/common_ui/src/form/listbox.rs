use std::cell::RefCell;

use internal_button::InternalButton;
use internal_options::InternalOptions;
use leptos::*;

use listbox_button::*;
use listbox_option::*;
use listbox_options::*;

pub use listbox_button::ListboxButton;
pub use listbox_option::ListboxOption;
pub use listbox_options::ListboxOptions;

mod internal_button;
mod internal_option;
mod internal_options;
mod listbox_button;
mod listbox_option;
mod listbox_options;

#[derive(Clone, Debug)]
struct ListboxContext<T: PartialEq + Clone + 'static> {
    pub id: usize,
    pub is_open: bool,
    pub current: T,
    pub focused: T,
    pub blur_lock: bool,
}

#[derive(Clone, Debug)]
enum ListboxDispatchAction<T: PartialEq + Clone + 'static> {
    Change(T),
    Open,
    Close(bool),
    SelectPrevious,
    SelectNext,
    Blur,
    BlurLock(bool),
}

#[derive(Clone, Debug)]
struct ListboxDispatch<T: PartialEq + Clone + 'static>(
    Callback<ListboxDispatchAction<T>>,
);

thread_local! {
    static NEXT_LISTBOX_ID: RefCell<usize> = RefCell::new(1);
}

#[component]
pub fn Listbox<T>(
    #[prop(into)] value: Signal<T>,
    #[prop(into)] on_change: Callback<T>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView
where
    T: PartialEq + std::fmt::Debug + Clone + 'static,
{
    let id = NEXT_LISTBOX_ID.with_borrow_mut(|value| {
        let id = *value;
        *value += 1;
        id
    });
    let fragment = children.expect("children")();
    let children = fragment.as_children();
    let button_ref = create_node_ref::<html::Button>();
    assert!(children.len() == 2);

    let button_definition: ListboxButtonDefinition = children.get(0).into();
    let options_definition: ListboxOptionsDefinition = children.get(1).into();
    let options: Vec<ListboxOptionDefinition<T>> = (&options_definition).into();

    let (context, set_context) = create_signal({
        let value = value.get_untracked();
        ListboxContext {
            id,
            is_open: false,
            current: value.clone(),
            focused: value,
            blur_lock: false,
        }
    });

    let dispatch = ListboxDispatch(Callback::new({
        let options = options.clone();
        move |action: ListboxDispatchAction<T>| match action {
            ListboxDispatchAction::Change(value) => {
                on_change.call(value);
                set_context.update(|context| {
                    context.is_open = false;
                });
                if let Some(button_ref) = button_ref.get() {
                    button_ref.focus().expect("focus button");
                }
            }
            ListboxDispatchAction::Open => {
                set_context.update(|context| {
                    context.is_open = true;
                });
            }
            ListboxDispatchAction::Close(focus_button) => {
                set_context.update(|context| {
                    context.is_open = false;
                    context.focused = context.current.clone();
                    if focus_button {
                        if let Some(button_ref) = button_ref.get() {
                            button_ref.focus().expect("button focus");
                        }
                    }
                });
            }
            ListboxDispatchAction::SelectPrevious => {
                let focused = context.get_untracked().focused;
                let focused_index = options
                    .iter()
                    .enumerate()
                    .find_map(|(index, option)| {
                        option.value.eq(&focused).then(|| index)
                    })
                    .expect("focused_index");

                let next_focused_index = if focused_index == 0 {
                    0
                } else {
                    focused_index - 1
                };
                let next_focused = options
                    .get(next_focused_index)
                    .expect("next focused element")
                    .value
                    .clone();

                set_context.update(move |context| {
                    context.focused = next_focused;
                    context.blur_lock = true;
                });
            }
            ListboxDispatchAction::SelectNext => {
                let focused = context.get_untracked().focused;
                let focused_index = options
                    .iter()
                    .enumerate()
                    .find_map(|(index, option)| {
                        option.value.eq(&focused).then(|| index)
                    })
                    .expect("focused_index");

                let next_focused_index =
                    (focused_index + 1).min(options.len() - 1);
                let next_focused = options
                    .get(next_focused_index)
                    .expect("next focused element")
                    .value
                    .clone();

                set_context.update(move |context| {
                    context.focused = next_focused;
                    context.blur_lock = true;
                });
            }
            ListboxDispatchAction::Blur => {
                if !context.get().blur_lock {
                    set_context.update(|context| {
                        context.is_open = false;
                        context.focused = context.current.clone();
                    });
                }
            }
            ListboxDispatchAction::BlurLock(value) => {
                set_context.update(|context| {
                    context.blur_lock = value;
                });
            }
        }
    }));

    provide_context(context);
    provide_context(dispatch.clone());

    create_effect(move |_| {
        let value = value.get();
        set_context.update(move |context| {
            context.current = value;
            context.is_open = false;
        });
    });

    let is_open = Signal::derive(move || context.get().is_open);

    view! {
        <div class=class>
            <InternalButton
                node_ref=button_ref
                definition=button_definition
                is_open=is_open
                on:click=move |_| {
                    let context = context.get();
                    let action = if context.is_open {
                        ListboxDispatchAction::Close(false)
                    } else {
                        ListboxDispatchAction::Open
                    };
                    dispatch.0.call(action)
                }

                on:mouseup=move |_| {
                    set_context
                        .update(|context| {
                            context.blur_lock = false;
                        });
                }

                on:mousedown=move |_| {
                    set_context
                        .update(|context| {
                            context.blur_lock = true;
                        });
                }

                on:mouseleave=move |_| {
                    set_context
                        .update(|context| {
                            context.blur_lock = false;
                        });
                }

                on:focusout=move |_| {
                    set_context
                        .update(|context| {
                            context.blur_lock = false;
                        });
                }
            />

            <InternalOptions
                class=options_definition.class.map(|class| class.to_string())
                options=options.clone()
            />
        </div>
    }
}
