use leptos::prelude::*;

use crate::{app_shell::AppShell, router::AppRouter};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <AppShell>
            <AppRouter />
        </AppShell>
    }
}
