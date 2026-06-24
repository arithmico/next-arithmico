use engine::{DocumentationItem, DocumentationItemType, Serialize};
use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};
use translate::FormattedMessage;
use ui::{common::page_title::PageTitle, container::page_header::PageHeader};
use web_state::WebState;

use crate::{components::PageWithSidebar, state::State};

#[derive(Debug, Clone, PartialEq, Params)]
struct ReferenceDetailsParams {
    endpoint_name: Option<String>,
}

#[component]
pub fn ReferenceDetailsPage() -> impl IntoView {
    let params = use_params::<ReferenceDetailsParams>();
    move || {
        params
            .get()
            .ok()
            .map(|params| params.endpoint_name)
            .flatten()
            .map(
                |endpoint_name| view! { <Inner endpoint_name=endpoint_name /> },
            )
    }
}

#[component]
fn Inner(endpoint_name: String) -> impl IntoView {
    let state = State::expect_state();
    let item = state.select({
        move |state| state.session.documentation().find_endpoint(&endpoint_name)
    });

    view! {
        <PageWithSidebar class="reference-details">
            {move || match item.get() {
                Some(item) => view! { <ItemSection item=item /> }.into_any(),
                None => view! { <h1>Not Found</h1> }.into_any(),
            }}

        </PageWithSidebar>
    }
}

#[component]
fn ItemSection(item: DocumentationItem) -> impl IntoView {
    let state = State::expect_state();
    let language = state.select(|state| state.settings.get_language());
    let context = state.select(|state| state.create_engine_context());
    let synopsis = Signal::derive({
        let item = item.clone();
        move || item.get_synopsis(&language.read()).cloned()
    });
    let description = Signal::derive({
        let item = item.clone();
        move || item.get_description(&language.read()).cloned()
    });
    view! {
        <PageHeader>
            <PageTitle>{synopsis}</PageTitle>
        </PageHeader>
        <p>{description}</p>
        {match item.get_documentation_type() {
            DocumentationItemType::Function => {
                view! {
                    <h2>
                        <FormattedMessage id="reference.item.parameters" />
                    </h2>
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
                                .cloned()
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
                                            <td>
                                                {
                                                    let param = param.clone();
                                                    move || param.get_requirement().serialize(&context.get())
                                                }
                                            </td>
                                            <td>
                                                {move || param.get_description(&language.read()).clone()}
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect_view()}
                        </tbody>
                    </table>
                    <h2>
                        <FormattedMessage id="reference.item.return_value" />
                    </h2>
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
            DocumentationItemType::Constant => ().into_any(),
        }}
    }
}
