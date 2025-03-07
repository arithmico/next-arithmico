use crate::{components::PageWithSidebar, state::State};
use history_header::HistoryHeader;
use history_list::HistoryList;
use leptos::prelude::*;
use web_state::WebState;

mod history_header;
mod history_item;
mod history_list;

#[component]
pub fn HistoryPage() -> impl IntoView {
    let state = State::expect_state();
    let items = state.select(|state| state.session.entries().to_vec());

    view! {
        <PageWithSidebar>
            <HistoryHeader />
            <HistoryList items=items />
        </PageWithSidebar>
    }
}
