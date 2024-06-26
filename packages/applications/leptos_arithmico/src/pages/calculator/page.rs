use crate::{
    components::*,
    state::{AppAction, AppState, Dispatcher},
};
use leptos::*;

#[component]
pub fn CalculatorPage() -> impl IntoView {
    let dispatch = expect_context::<Dispatcher>().0;
    let app_state = expect_context::<ReadSignal<AppState>>();
    let value = move || -> String {
        app_state.get().session.get_statements().last().map_or(
            String::new(),
            |statement| match statement.output.clone() {
                Ok(value) => value,
                Err(error) => error.to_string(),
            },
        )
    };

    view! {
        <PageWithSidebar>
            <h1 class="text-3xl font-bold">Calculator</h1>
            <input
                class="border border-black"
                type="text"
                on:change=move |event| {
                    dispatch.call(AppAction::Evaluate(event_target_value(&event)))
                }
            />

            <input readonly value=value/>
        </PageWithSidebar>
    }
}
