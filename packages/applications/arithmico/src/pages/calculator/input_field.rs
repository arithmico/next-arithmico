use crate::utils::expect_dispatch;
use ev::InputEvent;
use input_action::InputAction;
use leptos::*;
use node_utils::target_node;

mod absolute_range;
mod input_action;
mod node_utils;
mod static_range;

#[component]
pub fn InputField() -> impl IntoView {
    let _dispatch = expect_dispatch();
    let (input, set_input) = create_signal(String::new());

    view! {
        <div
            contenteditable
            data-testid="calculator-input"
            class="p-2 text-xl bg-white rounded-sm border outline-none focus-visible:border-black border-neutral-300"
            on:beforeinput=move |event| before_input(event, input, set_input)
        >
            {move || input.get()}
        </div>
    }
}

fn before_input(
    event: InputEvent,
    input: ReadSignal<String>,
    set_input: WriteSignal<String>,
) {
    event.prevent_default();
    let action = InputAction::try_from(&event).expect("input action");
    let (new_input, range) = action
        .apply(input.get_untracked().as_str())
        .expect("action result");
    set_input.set(new_input);

    let selection = document()
        .get_selection()
        .expect("selection")
        .expect("selection");
    selection.remove_all_ranges().expect("remove all ranges");
    selection
        .add_range(&range.into_range(&target_node(&event)).expect("range"))
        .expect("set selection");

    //logging::log!("result: {:#?}", result);
    //logging::log!("range: {:#?}", result.1.into_range(&target_node(&event)));
}
