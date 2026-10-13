use leptos::prelude::*;
use translate::FormattedMessage;

use crate::{components::CalculatorOutput, state::LogEntry};

#[component]
pub fn HistoryItem(item: LogEntry, position: usize) -> impl IntoView {
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
                    {
                        let item = item.clone();
                        move || {
                            view! { <CalculatorOutput value=item.output.clone() /> }
                        }
                    }
                </dd>
            </dl>
        </li>
    }
}
