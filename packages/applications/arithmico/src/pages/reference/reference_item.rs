use engine::{DocumentationItem, DocumentationItemType, Language, Serialize};
use leptos::prelude::*;
use translate::FormattedMessage;
use web_state::WebState;

use crate::state::State;

#[component]
pub fn ReferenceItem(
    item: DocumentationItem,
    language: Language,
) -> impl IntoView {
    let state = State::expect_state();
    let context: Signal<engine::Context> = state.select(|state| {
        let decimal_places = state.settings.decimal_places;
        let decimal_format = state
            .settings
            .override_decimal_format
            .decimal_format()
            .cloned()
            .unwrap_or_default();

        state.session.create_context(decimal_places, decimal_format)
    });

    view! {
        {match item.get_documentation_type() {
            DocumentationItemType::Function => {
                view! {
                    <h3>{item.get_synopsis(&language).cloned()}</h3>
                    <section>
                        <h3>
                            <FormattedMessage id="reference.item.description" />
                        </h3>
                        <p>
                            {item
                                .get_description(&language)
                                .cloned()
                                .unwrap_or_default()}
                        </p>
                        <h3>
                            <FormattedMessage id="reference.item.parameters" />
                        </h3>
                        <table>
                            <thead>
                                <tr>
                                    <th>
                                        <FormattedMessage id="reference.item.parameter.name" />
                                    </th>
                                    <th>
                                        <FormattedMessage id="reference.item.parameter.type" />
                                    </th>
                                    <th>
                                        <FormattedMessage id="reference.item.parameter.requirement" />
                                    </th>
                                    <th>
                                        <FormattedMessage id="reference.item.parameter.description" />
                                    </th>
                                </tr>
                            </thead>
                            <tbody>
                                {item
                                    .get_parameters()
                                    .iter()
                                    .map(|param| {
                                        view! {
                                            <tr>
                                                <td>{param.get_name().clone()}</td>
                                                <td>
                                                    {param
                                                        .get_parameter_types()
                                                        .iter()
                                                        .map(|node| node.to_string())
                                                        .collect::<Vec<String>>()
                                                        .join(", ")}
                                                </td>
                                                <td>{param.get_requirement().serialize(&context.get())}</td>
                                                <td>{param.get_description(&language).clone()}</td>
                                            </tr>
                                        }
                                    })
                                    .collect_view()}
                            </tbody>
                        </table>
                        <h3>
                            <FormattedMessage id="reference.item.return_value" />
                        </h3>
                        <p>
                            {item
                                .get_return_types()
                                .iter()
                                .map(|node| node.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")}
                        </p>
                    </section>
                }
                    .into_any()
            }
            DocumentationItemType::Constant => {
                view! {
                    <h3>{item.get_synopsis(&language).cloned()}</h3>
                    <section>
                        <h3>
                            <FormattedMessage id="reference.item.description" />
                        </h3>
                        <p>
                            {item
                                .get_description(&language)
                                .cloned()
                                .unwrap_or_default()}
                        </p>
                    </section>
                }
                    .into_any()
            }
        }}
    }
}
