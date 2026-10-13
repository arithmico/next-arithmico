use leptos::prelude::*;
use node::{FunctionCall, Node, Symbol};
use web_state::use_state;

use crate::{components::DisplayNode, state::State};

#[component]
pub fn Definitions() -> impl IntoView {
    let state = use_state::<State>();
    let definitions = state.query(|state| state.symbols.clone());

    move || {
        let definitions = definitions.get();
        view! {
            <ul class="definitions-list">
                {definitions
                    .into_iter()
                    .map(|(key, value)| -> (Node, Node) {
                        if let Node::Function(node) = value {
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
                        }
                    })
                    .map(|(key, value)| {

                        view! {
                            <li>
                                <span>
                                    <DisplayNode node=key />
                                </span>
                                <span>{":="}</span>
                                <span>
                                    <DisplayNode node=value />
                                </span>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        }
    }
}
