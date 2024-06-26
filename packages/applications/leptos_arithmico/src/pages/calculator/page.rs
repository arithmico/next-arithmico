use crate::{
    components::*,
    state::{DispatchAction, Dispatcher},
};
use engine::Session;
use leptos::*;

#[component]
pub fn CalculatorPage() -> impl IntoView {
    let dispatch = expect_context::<Dispatcher>().0;
    let session = expect_context::<ReadSignal<Session>>();
    let value = move || -> String {
        session.get().get_statements().last().map_or(
            String::new(),
            |statement| match statement.output.clone() {
                Ok(value) => value,
                Err(error) => error.to_string(),
            },
        )
    };

    view! {
        <Page>
            <h1 class="text-3xl font-bold">Calculator</h1>

            <input
                class="border border-black"
                type="text"
                on:change=move |event| {
                    dispatch.call(DispatchAction::Evaluate(event_target_value(&event)))
                }
            />

            <input readonly value=value/>

        </Page>
    }
}
