use crate::{components::PageWithSidebar, state::State};
use leptos::prelude::*;
use web_state::WebState;
use definitions_header::DefinitionsHeader;
use definitions_list::DefinitionList;

mod definitions_header;
mod definitions_list;

#[component]
pub fn DefinitionsPage() -> impl IntoView {
    let state = State::expect_state();
    let session = state.select(|state| state.session);
    let decimal_format = state.select(|state| state.settings.override_decimal_format.decimal_format().unwrap().clone());
    let decimal_places = state.select(|state| state.settings.decimal_places);

    view! {
        <PageWithSidebar>
            <DefinitionsHeader />
            <DefinitionList
                session=session
                decimal_format=decimal_format
                decimal_places=decimal_places
            />
        </PageWithSidebar>
    }
}
