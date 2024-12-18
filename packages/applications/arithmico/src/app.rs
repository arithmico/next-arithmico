use leptos::prelude::*;

use crate::{app_shell::AppShell, router::Router};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <AppShell>
            <Router />
        </AppShell>
    }
}
