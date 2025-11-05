use leptos::prelude::*;
use engine::{Language, DocumentationItem};
use translate::FormattedMessage;

#[component]
pub fn ReferenceItemInformation(item: DocumentationItem) -> impl IntoView {
    view! {
        <dl>
            <dt>
                <FormattedMessage id="reference.item.description" />
            </dt>
            <dd>{item.description(&Language::English).cloned().unwrap()}</dd>
            <dt>
                <FormattedMessage id="reference.item.parameters" />
            </dt>
            <dd>
                <ul>
                    {item
                        .parameters()
                        .iter()
                        .map(|param| {
                            view! {
                                <li>{param.name()}</li>
                                <li>{param.description(&Language::English)}</li>
                            }
                        })
                        .collect_view()}
                </ul>
            </dd>
            <dt>
                <FormattedMessage id="reference.item.return_value" />
            </dt>
            <dd>{item.return_type().to_string()}</dd>
        </dl>
    }
}