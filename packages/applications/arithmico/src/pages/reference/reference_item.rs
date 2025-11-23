use leptos::prelude::*;
use engine::{Language, DocumentationItem};
use translate::FormattedMessage;

#[component]
pub fn ReferenceItem(item: DocumentationItem, language: Language) -> impl IntoView {
    view! {
        <dt>{item.get_synopsis(&language).cloned()}</dt>
        <dd>
            <dl>
                <dt>
                    <FormattedMessage id="reference.item.description" />
                </dt>
                <dd>{item.get_description(&language).cloned().unwrap()}</dd>
                <dt>
                    <FormattedMessage id="reference.item.parameters" />
                </dt>
                <dd>
                    <ul>
                        {item
                            .get_parameters()
                            .iter()
                            .map(|param| {
                                view! {
                                    <li>{param.get_name().clone()}</li>
                                    <li>{param.get_description(&language).clone()}</li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </dd>
                <dt>
                    <FormattedMessage id="reference.item.return_value" />
                </dt>
                <dd>{item.get_return_type().to_string()}</dd>
            </dl>
        </dd>
    }
}