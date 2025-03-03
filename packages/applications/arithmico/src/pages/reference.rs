use crate::{components::*, state::State};
use leptos::prelude::*;
use reference_header::ReferenceHeader;
use reference_section::ReferenceSection;
use web_state::WebState;

mod reference_header;
mod reference_section;

#[component]
pub fn ReferencePage() -> impl IntoView {
    let state = State::expect_state();
    let modules = state.select(|state| {
        state
            .session
            .documentation()
            .modules()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    });

    view! {
        <PageWithSidebar>
            <ReferenceHeader />
            {move || {
                modules
                    .get()
                    .into_iter()
                    .map(|module| {
                        view! { <ReferenceSection module=module /> }
                    })
                    .collect_view()
            }}

        </PageWithSidebar>
    }
}
