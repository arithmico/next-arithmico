use leptos::prelude::*;

use crate::{
    components::PageWithSidebar,
    pages::definitions::{definitions::Definitions, header::Header},
};

mod definitions;
mod header;

#[component]
pub fn DefinitionsPage() -> impl IntoView {
    view! {
        <PageWithSidebar class="definitions-page">
            <Header />
            <Definitions />
        </PageWithSidebar>
    }
}
