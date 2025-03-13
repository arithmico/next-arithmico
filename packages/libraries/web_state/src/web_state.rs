use leptos::prelude::*;

use crate::{WebStateAction, web_state_provider::WebStateWrapper};

pub trait WebState: Clone + Send + Sync + 'static {
    fn expect_state() -> WebStateApi<Self> {
        let context = expect_context::<RwSignal<WebStateWrapper<Self>>>();
        WebStateApi { context }
    }
}

#[derive(Clone)]
pub struct WebStateApi<S: WebState> {
    context: RwSignal<WebStateWrapper<S>>,
}

impl<S: WebState> Copy for WebStateApi<S> {}

impl<S: WebState> WebStateApi<S> {
    pub fn dispatch<A: WebStateAction<S>>(&self, action: &A) {
        self.context
            .update(move |WebStateWrapper(state)| action.apply(state));
    }

    pub fn dispatch_untracked<A: WebStateAction<S>>(&self, action: &A) {
        self.context
            .update_untracked(move |WebStateWrapper(state)| {
                action.apply(state)
            });
    }

    pub fn select<
        F: Fn(S) -> T + Send + Sync + 'static,
        T: Send + Sync + 'static,
    >(
        &self,
        selector: F,
    ) -> Signal<T> {
        let context = self.context;
        Signal::derive(move || {
            let state = context.get().0;
            selector(state)
        })
    }

    pub fn get(&self) -> S {
        self.context.get().0
    }
}
