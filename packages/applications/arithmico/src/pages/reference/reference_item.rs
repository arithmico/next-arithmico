use engine::{DocumentationItem, Language};
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};
use translate::FormattedMessage;

#[component]
pub fn ReferenceItem(
    item: DocumentationItem,
    language: Language,
) -> impl IntoView {
    let navigate = use_navigate();
    let url = format!("/reference/{}", item.get_endpoint_name());

    view! {
        <li
            class="reference-item"
            on:click=move |_| {
                navigate(&url, Default::default());
            }
        >
            <h3>{item.get_synopsis(&language).cloned()}</h3>
            <div>
                <p>
                    {item
                        .get_description(&language)
                        .cloned()
                        .unwrap_or_default()}
                </p>
                <A href=url.clone() {..} class="sr-only">
                    <FormattedMessage id="reference.item.details" />
                </A>
            </div>
        </li>
    }
}

/*
#[component]
pub fn ReferenceItem(
    item: DocumentationItem,
    language: Language,
) -> impl IntoView {
    let state = State::expect_state();
    let context = state.select(|state| state.create_engine_context());

    view! {
        <section class="item-section">
            {match item.get_documentation_type() {
                DocumentationItemType::Function => {
                    view! {
                        <h3>{item.get_synopsis(&language).cloned()}</h3>
                        <h4>
                            <FormattedMessage id="reference.item.description" />
                        </h4>
                        <p>
                            {item
                                .get_description(&language)
                                .cloned()
                                .unwrap_or_default()}
                        </p>
                        <h4>
                            <FormattedMessage id="reference.item.parameters" />
                        </h4>
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
                        <h4>
                            <FormattedMessage id="reference.item.return_value" />
                        </h4>
                        <p>
                            {item
                                .get_return_types()
                                .iter()
                                .map(|node| node.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")}
                        </p>
                    }
                        .into_any()
                }
                DocumentationItemType::Constant => {
                    view! {
                        <h3>{item.get_synopsis(&language).cloned()}</h3>
                        <h4>
                            <FormattedMessage id="reference.item.description" />
                        </h4>
                        <p>
                            {item
                                .get_description(&language)
                                .cloned()
                                .unwrap_or_default()}
                        </p>
                    }
                        .into_any()
                }
            }}
        </section>
    }
}
*/
