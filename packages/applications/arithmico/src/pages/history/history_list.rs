use leptos::prelude::*;
use web_state::use_state;

use crate::{pages::history::history_item::HistoryItem, state::State};

#[component]
pub fn HistoryList() -> impl IntoView {
    let state = use_state::<State>();
    let items = state.query(|state| state.log.entries().to_vec());

    view! {
        <ul class="history-list">
            {move || {
                let items = items.get();
                items
                    .into_iter()
                    .enumerate()
                    .map(|(position, item)| {
                        view! { <HistoryItem item=item position=position /> }
                    })
                    .collect_view()
            }}
        </ul>
    }
}
