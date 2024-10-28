use leptos::*;

use crate::state::{AppAction, AppState};

#[derive(Clone)]
pub struct Dispatch(pub Callback<AppAction>);

#[component]
pub fn StateProvider(children: Children) -> impl IntoView {
    let (app_state, set_app_state) = create_signal(AppState::load_or_default());

    provide_context::<Dispatch>(Dispatch(Callback::<AppAction>::new(
        move |action| {
            set_app_state.update(|app_state| app_state.reduce(action))
        },
    )));

    provide_context(app_state);

    view! { <>{children()}</> }
}
