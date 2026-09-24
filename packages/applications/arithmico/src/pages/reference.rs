use crate::{
    components::*, pages::reference::matching::match_reference_modules,
    state::State,
};
use leptos::prelude::*;
use reference_header::ReferenceHeader;
use reference_section::ReferenceSection;
use translate::use_language;
use web_state::WebState;

mod matching;
mod reference_header;
mod reference_item;
mod reference_section;

#[component]
pub fn ReferencePage() -> impl IntoView {
    let state = State::expect_state();
    let language = use_language();
    let search_query = RwSignal::new(String::new());
    let modules = state.select(move |state| {
        let mut items = state
            .session
            .documentation()
            .modules()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();

        let language = language.get();
        items.sort_by(|a, b| a.name(language).cmp(&b.name(language)));
        items
    });
    let search_results =
        match_reference_modules(modules, language, search_query.into());

    view! {
        <PageWithSidebar>
            <ReferenceHeader search_query=search_query />
            {move || {
                search_results
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
