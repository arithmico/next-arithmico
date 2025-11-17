use leptos::prelude::*;

use crate::{
    app::{
        state::StateProvider, theme::ThemeProvider,
        translation::TranslationProvider,
    },
    pages::ApplicationRouter,
};

mod state;
mod theme;
mod translation;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <StateProvider>
            <ThemeProvider>
                <TranslationProvider>
                    <ApplicationRouter />
                </TranslationProvider>
            </ThemeProvider>
        </StateProvider>
    }
}
