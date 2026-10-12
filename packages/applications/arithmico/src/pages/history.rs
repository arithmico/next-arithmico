use crate::components::PageWithSidebar;
use history_header::HistoryHeader;
use history_list::HistoryList;
use leptos::prelude::*;

mod history_header;
mod history_item;
mod history_list;

#[component]
pub fn HistoryPage() -> impl IntoView {
    view! {
        <PageWithSidebar>
            <HistoryHeader />
            <HistoryList />
        </PageWithSidebar>
    }
}
