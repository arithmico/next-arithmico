use leptos::*;
use state::StateProvider;
use translation::TranslationProvider;

mod state;
mod translation;

pub use state::Dispatch;

#[component]
pub fn AppShell(children: Children) -> impl IntoView {
    view! {
        <StateProvider>
            <TranslationProvider>{children()}</TranslationProvider>
        </StateProvider>
    }
}
