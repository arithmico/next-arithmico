use leptos::prelude::*;
use state::StateProvider;
use theme::ThemeProvider;
use translation::TranslationProvider;

mod state;
mod theme;
mod translation;

pub use state::Dispatch;

#[component]
pub fn AppShell(children: Children) -> impl IntoView {
    view! {
        <StateProvider>
            <ThemeProvider>
                <TranslationProvider>{children()}</TranslationProvider>
            </ThemeProvider>
        </StateProvider>
    }
}
