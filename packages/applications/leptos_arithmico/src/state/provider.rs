use engine::Settings;
use leptos::*;

use crate::state::AppState;

#[derive(Clone)]
pub enum DispatchAction {
    Evaluate(String),
}

#[derive(Clone)]
pub struct Dispatcher(pub Callback<DispatchAction>);

#[component]
pub fn StateProvider(children: Children) -> impl IntoView {
    let (app_state, set_app_state) = create_signal(AppState::new());

    provide_context::<Dispatcher>(Dispatcher(Callback::<DispatchAction>::new(
        move |action| {
            match action {
                DispatchAction::Evaluate(input) => {
                    let next_session = app_state
                        .get()
                        .session
                        .push(&input, &Settings::default());
                    set_app_state.set(AppState {
                        session: next_session,
                    });
                }
            };
        },
    )));

    provide_context(app_state);

    view! { <>{children()}</> }
}
