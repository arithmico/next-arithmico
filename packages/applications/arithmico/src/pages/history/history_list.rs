use engine::SessionEntry;
use leptos::prelude::*;

use crate::pages::history::history_item::HistoryItem;

#[component]
pub fn HistoryList(items: Signal<Vec<SessionEntry>>) -> impl IntoView {
    view! {
        <ul class="history-list">
            {move || {
                let items = items.get();
                items
                    .into_iter()
                    .map(|item| {
                        view! { <HistoryItem item=item /> }
                    })
                    .collect_view()
            }}
        </ul>
    }
}
