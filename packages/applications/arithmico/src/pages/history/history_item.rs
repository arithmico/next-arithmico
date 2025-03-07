use engine::SessionEntry;
use leptos::prelude::*;
use translate::FormattedMessage;

use crate::components::CalculatorOutput;

#[component]
pub fn HistoryItem(item: SessionEntry) -> impl IntoView {
    view! {
        <li class="history-item">
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
