use engine::SessionEntry;
use leptos::prelude::*;
use translate::FormattedMessage;

use crate::components::CalculatorOutput;

#[component]
pub fn HistoryItem(item: SessionEntry, position: usize) -> impl IntoView {
    view! {
        <li class="history-item">
            <span class="history-item-id">{format!("#{}", position)}</span>
            <dl>
                <dt>
                    <FormattedMessage id="history.input" />
                </dt>
                <dd>{item.input.clone()}</dd>
                <dt>
                    <FormattedMessage id="history.output" />
                </dt>
                <dd>
                    <CalculatorOutput value=Some(item.output.clone()) />
                </dd>
            </dl>
        </li>
    }
}
