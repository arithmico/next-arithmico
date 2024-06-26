use leptos::*;

use crate::state::{AppAction, AppState};

#[derive(Clone)]
pub struct Dispatch(pub Callback<AppAction>);

#[component]
pub fn StateProvider(children: Children) -> impl IntoView {
    let (app_state, set_app_state) = create_signal(AppState::new());

    provide_context::<Dispatch>(Dispatch(Callback::<AppAction>::new(
        move |action| set_app_state.set(app_state.get().reduce(action)),
    )));

    provide_context(app_state);

    view! { <>{children()}</> }
}
