use leptos::prelude::*;

use engine::{SerializeNode, SerializeOptions, SessionError};
use node::Node;

use crate::components::CalculatorErrorOutput;

#[component]
pub fn CalculatorOutput(
    #[prop(into)] value: Signal<Option<Result<Node, SessionError>>>,
    #[prop(into)] options: Signal<SerializeOptions>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <output
            for="calculator-output"
            data-testid="calculator-output"
            class=format!("calculator-output {}", class.unwrap_or_default())
        >
            {move || {
                let options = options.get();
                match value.get() {
                    Some(result) => {
                        match result {
                            Ok(node) => {
                                match node {
                                    Node::DataFrame(data_frame) => {
                                        view! {
                                            <table class="data-frame">
                                                {
                                                    let headers = data_frame.headers.clone();
                                                    (!data_frame.headers_empty())
                                                        .then(move || {
                                                            view! {
                                                                <thead>
                                                                    {headers
                                                                        .into_iter()
                                                                        .map(|header| {
                                                                            header
                                                                                .map(|header| {
                                                                                    view! {
                                                                                        <th>
                                                                                            <SerializedNode node=header options=options />
                                                                                        </th>
                                                                                    }
                                                                                })
                                                                        })
                                                                        .collect_view()}
                                                                </thead>
                                                            }
                                                        })
                                                }
                                                <tbody>
                                                    {data_frame
                                                        .rows()
                                                        .map(|row| {
                                                            view! {
                                                                <tr>
                                                                    {row
                                                                        .iter()
                                                                        .cloned()
                                                                        .map(|node| {
                                                                            node.map(|node| {
                                                                                view! {
                                                                                    <td>
                                                                                        <SerializedNode node=node options=options />

                                                                                    </td>
                                                                                }
                                                                            })
                                                                        })
                                                                        .collect_view()}
                                                                </tr>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </tbody>
                                            </table>
                                        }
                                            .into_any()
                                    }
                                    node => {
                                        view! { <SerializedNode node=node options=options /> }
                                            .into_any()
                                    }
                                }
                            }
                            Err(error) => {
                                // TODO: translate serialization error
                                view! { <CalculatorErrorOutput error=error /> }
                                    .into_any()
                            }
                        }
                    }
                    None => view! { <>" "</> }.into_any(),
                }
            }}
        </output>
    }
}

#[component]
fn SerializedNode(node: Node, options: SerializeOptions) -> impl IntoView {
    node.serialize(options)
        .unwrap_or_else(|_| String::from("SerializationError"))
        .into_any()
}
