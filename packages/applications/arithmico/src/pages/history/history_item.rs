use engine::SessionEntry;
use leptos::prelude::*;
use translate::FormattedMessage;
use web_state::WebState;

use crate::{components::CalculatorOutput, state::State};

#[component]
pub fn HistoryItem(item: SessionEntry, position: usize) -> impl IntoView {
    let state = State::expect_state();
    let context = state.select(|state| state.create_engine_context());

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
                            let context = context.get();
                            view! {
                                <CalculatorOutput
                                    value=Some(item.output.clone())
                                    context=context
                                />
                            }
                        }
                    }
                </dd>
            </dl>
        </li>
    }
}
