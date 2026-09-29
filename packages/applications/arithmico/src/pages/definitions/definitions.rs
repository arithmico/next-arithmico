use engine::{SerializeError, SerializeNode, SerializeOptions};
use leptos::prelude::*;
use node::{FunctionCall, Node, Symbol};
use web_state::WebState;

use crate::state::State;

#[component]
pub fn Definitions() -> impl IntoView {
    let state = State::expect_state();
    let settings = state.select(|state| state.settings);
    let definitions = state.select(|state| state.session.stack_entries());

    move || {
        let settings = settings.get();
        let definitions = definitions.get();
        view! {
            <ul class="definitions-list">
                {definitions
                    .into_iter()
                    .map(|
                        (key, value),
                    | -> Result<(String, String), SerializeError> {
                        let (key_node, value_node) = if let Node::Function(node) = value {
                            let key_node = FunctionCall::new(
                                Symbol::new(&key),
                                node
                                    .signature
                                    .arguments()
                                    .iter()
                                    .map(|argument| Symbol::new(argument.get_name()))
                                    .collect(),
                            );
                            let value_node = *node.expression.clone();
                            (key_node, value_node)
                        } else {
                            (Symbol::new(&key), value)
                        };
                        let decimal_places = settings.decimal_places;
                        let language = settings
                            .override_decimal_format
                            .decimal_format()
                            .unwrap_or(settings.get_language());
                        let options = SerializeOptions::new(
                            language,
                            decimal_places,
                        );
                        let key_string = key_node.serialize(options)?;
                        let value_string = value_node.serialize(options)?;
                        Ok((key_string, value_string))
                    })
                    .map(|entry: Result<(String, String), _>| {
                        entry
                            .map(|(key, value)| {
                                view! {
                                    <li>
                                        <span>{key}</span>
                                        <span>{":="}</span>
                                        <span>{value}</span>
                                    </li>
                                }
                            })
                    })
                    .collect_view()}
            </ul>
        }
    }
}
