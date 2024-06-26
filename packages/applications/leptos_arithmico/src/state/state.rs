use engine::{load_host_api, Session, Settings};
use leptos::*;

#[derive(Clone)]
pub enum DispatchAction {
    Evaluate(String),
}

#[derive(Clone)]
pub struct Dispatcher(pub Callback<DispatchAction>);

#[component]
pub fn StateProvider(children: Children) -> impl IntoView {
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

    view! { <>{children()}</> }
}
