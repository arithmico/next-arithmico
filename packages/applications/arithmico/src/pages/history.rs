use history_list::HistoryList;
use leptos::prelude::*;
use translate::FormattedMessage;
use ui::common::page_title::PageTitle;
use web_state::WebState;

use crate::{components::PageWithSidebar, state::State};

mod history_item;
mod history_list;

#[component]
pub fn HistoryPage() -> impl IntoView {
    let state = State::expect_state();
    let items = state.select(|state| state.session.entries().to_vec());

    view! {
        <PageWithSidebar>
            <PageTitle>
                <FormattedMessage id="history.title" />
            </PageTitle>
            <HistoryList items=items />
        </PageWithSidebar>
    }
}
