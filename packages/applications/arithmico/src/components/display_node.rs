use leptos::prelude::*;
use node::Node;
use serializer::SerializeNode;
use web_state::use_state;

use crate::state::State;

#[component]
pub fn DisplayNode(node: Node) -> impl IntoView {
    let state = use_state::<State>();
    let options = state.query(|state| -> serializer::Options {
        state.create_serialize_options()
    });

    match node {
        Node::DataFrame(data_frame) => {
            view! {
                <table class="data-frame">
                    {
                        let headers = data_frame.headers.clone();
                        (!data_frame.headers_empty())
                            .then(move || {
                                let options = options.get();
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
                        {
                            let options = options.get();
                            move || {
                                data_frame
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
                                    .collect_view()
                            }
                        }
                    </tbody>
                </table>
            }
                .into_any()
        }
        node => {
            (move || {
                let options = options.get(); 
                view! { <SerializedNode node=node.clone() options=options /> }
            })
                .into_any()
        }
    }
}

#[component]
fn SerializedNode(node: Node, options: serializer::Options) -> impl IntoView {
    node.serialize(options)
        .unwrap_or_else(|_| String::from("SerializationError"))
}
