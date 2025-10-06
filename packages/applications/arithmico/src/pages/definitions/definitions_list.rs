use engine::{serialize_node, DecimalFormat, DecimalPlaces, Session};
use leptos::prelude::*;

#[component]
pub fn DefinitionList(session: Signal<Session>, decimal_format: Signal<DecimalFormat>, decimal_places: Signal<DecimalPlaces>) -> impl IntoView {
    view! {
        <ul class="definitions-list">
            {move || {
                let session = session.get();
                session.stack().entries()
                    .into_iter()
                    .map(|(name, node)| {
                        let context = session.create_context(decimal_places.get(), decimal_format.get());
    
                        view! {
                            <li>
                            {
                                format!("{} := {}", name, serialize_node(&node, &context).unwrap_or(String::from("failed")))
                            }
                            </li>
                        }
                    })
                    .collect_view()
            }}
        </ul>
    }
}
