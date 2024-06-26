use engine::{load_host_api, Session, Settings};
use leptos::*;

use crate::{
    router::Router,
    state::{DispatchAction, Dispatcher},
};

#[component]
pub fn App() -> impl IntoView {
    let (session, set_session) =
        create_signal(Session::new(std::rc::Rc::new(load_host_api())));

    provide_context::<Dispatcher>(Dispatcher(Callback::<DispatchAction>::new(
        move |action| {
            match action {
                DispatchAction::Evaluate(input) => {
                    let next_session =
                        session.get().push(&input, &Settings::default());
                    set_session.set(next_session);
                }
            };
        },
    )));

    provide_context(session);

    view! { <Router/> }
}
