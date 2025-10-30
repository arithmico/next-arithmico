use leptos::prelude::*;

use crate::{components::PageWithSidebar, pages::definitions::header::Header};

mod header;

#[component]
pub fn DefinitionsPage() -> impl IntoView {
    view! {
        <PageWithSidebar>
            <Header />
        </PageWithSidebar>
    }
}
