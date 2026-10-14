use std::ops::{Deref, DerefMut};

use leptos::prelude::*;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("State not found.")]
    StateNotFound,
}

#[derive(Debug)]
pub struct State<T>(pub RwSignal<T>);

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for State<T> {}

impl<T> State<T>
where
    T: 'static + Send + Sync,
{
    pub fn dispatch<M: StateMutation<T> + Send + Sync>(&self, mutation: M) {
        let Some(mut state) = self.0.try_write() else {
            return;
        };
        mutation.apply(state.deref_mut());
    }

    pub fn dispatch_untracked<M: StateMutation<T> + Send + Sync>(
        &self,
        mutation: M,
    ) {
        let Some(mut state) = self.0.try_write_untracked() else {
            return;
        };
        mutation.apply(state.deref_mut());
    }

    pub fn query<O, Q>(&self, query: Q) -> Signal<O>
    where
        O: Send + Sync + 'static,
        Q: Fn(&T) -> O + Send + Sync + 'static,
    {
        let state = self.0;
        Signal::derive(move || {
            let state = state.read();
            query(state.deref())
        })
    }

    pub fn query_memo<O, Q>(&self, query: Q) -> Memo<O>
    where
        O: PartialEq + Send + Sync + 'static,
        Q: Fn(&T) -> O + Send + Sync + 'static,
    {
        let state = self.0;
        Memo::new(move |_| {
            let state = state.read();
            query(state.deref())
        })
    }

    pub fn query_untracked<O, Q>(&self, query: Q) -> Signal<O>
    where
        O: Send + Sync + 'static,
        Q: Fn(&T) -> O + Send + Sync + 'static,
    {
        let state = self.0;
        Signal::derive(move || {
            let state = state.read_untracked();
            query(state.deref())
        })
    }
}

pub trait StateMutation<State> {
    fn apply(&self, state: &mut State);
}

pub fn use_state_or_err<S: Send + Sync + 'static>() -> Result<State<S>, Error> {
    use_context::<State<S>>().ok_or(Error::StateNotFound)
}

pub fn use_state<S: Send + Sync + 'static>() -> State<S> {
    expect_context::<State<S>>()
}

#[component]
pub fn StateProvider<S: Send + Sync + 'static>(
    state: RwSignal<S>,
    children: Children,
) -> impl IntoView {
    provide_context(State(state));
    view! { <>{children()}</> }
}
