use leptos::prelude::*;
use translate::FormattedMessage;
use web_state::use_state;

use crate::{
    components::CalculatorOutput,
    state::{LogEntry, State},
};

#[component]
pub fn HistoryItem(item: LogEntry, position: usize) -> impl IntoView {
    let state = use_state::<State>();
    let context = state.query(|state| state.create_serialize_options());

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
                                    value=item.output.clone()
                                    options=context
                                />
                            }
                        }
                    }
                </dd>
            </dl>
        </li>
    }
}
