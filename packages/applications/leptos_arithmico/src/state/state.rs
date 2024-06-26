use leptos::*;

#[derive(Clone)]
pub enum DispatchAction {
    Evaluate(String),
}

#[derive(Clone)]
pub struct Dispatcher(pub Callback<DispatchAction>);
