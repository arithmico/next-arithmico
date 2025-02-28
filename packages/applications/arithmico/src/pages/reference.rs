use crate::{components::*, utils::use_app_state};
use leptos::prelude::*;
use reference_header::ReferenceHeader;
use reference_section::ReferenceSection;

mod reference_header;
mod reference_section;

#[component]
pub fn ReferencePage() -> impl IntoView {
    let app_state = use_app_state();
    let modules = Signal::derive(move || {
        app_state
            .get()
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
