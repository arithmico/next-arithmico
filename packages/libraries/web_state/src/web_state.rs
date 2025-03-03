use leptos::prelude::*;

use crate::{web_state_provider::WebStateWrapper, WebStateAction};

pub trait WebState: Clone + Send + Sync + 'static {
    fn expect_state() -> WebStateApi<Self> {
        let context = expect_context::<RwSignal<WebStateWrapper<Self>>>();
        WebStateApi { context }
    }
}

#[derive(Clone, Copy)]
pub struct WebStateApi<S: WebState> {
    context: RwSignal<WebStateWrapper<S>>,
}

impl<S: WebState> WebStateApi<S> {
    pub fn dispatch<A: WebStateAction<S>>(&self, action: &A) {
        let mut state = self.context.get_untracked().0;
        action.apply(&mut state);
        self.context.set(WebStateWrapper(state));
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
}
