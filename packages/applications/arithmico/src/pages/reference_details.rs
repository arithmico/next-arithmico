use std::collections::HashMap;

use engine_protocol::{DocumentationItem, DocumentationItemType};
use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};
use serializer::SerializeNode;
use translate::FormattedMessage;
use ui::{
    common::breadcrumbs::{Breadcrumbs, BreadcrumbsItem},
    container::page_header::PageHeader,
};
use web_state::use_state;

use crate::{
    components::{DisplayNode, PageWithSidebar},
    state::State,
};

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
            .and_then(|params| params.endpoint_name)
            .map(
                |endpoint_name| view! { <Inner endpoint_name=endpoint_name /> },
            )
    }
}

#[component]
fn Inner(endpoint_name: String) -> impl IntoView {
    let state = use_state::<State>();
    let item = state.query({
        let endpoint_name = endpoint_name.clone();
        move |state| state.documentation.find_endpoint(&endpoint_name)
    });

    view! {
        <PageWithSidebar class="reference-details">
            {
                let endpoint_name = endpoint_name.clone();
                move || match item.get() {
                    Some(item) => {
                        view! {
                            <ItemSection
                                item=item
                                endpoint_name=endpoint_name.clone()
                            />
                        }
                            .into_any()
                    }
                    None => view! { <h1>Not Found</h1> }.into_any(),
                }
            }

        </PageWithSidebar>
    }
}

#[component]
fn ItemSection(
    item: DocumentationItem,
    endpoint_name: String,
) -> impl IntoView {
    let state = use_state::<State>();
    let language = state.query(|state| state.settings.get_language());
    let options = state.query(|state| state.create_serialize_options());
    let synopsis = Signal::derive({
        let item = item.clone();
        move || item.get_synopsis().clone()
    });
    let description = Signal::derive({
        let item = item.clone();
        move || item.get_description(language.get())
    });

    view! {
        <PageHeader>
            <Breadcrumbs>
                <BreadcrumbsItem href="/reference">
                    <FormattedMessage id="reference.title" />
                </BreadcrumbsItem>
                <BreadcrumbsItem
                    href=format!("/reference/{}", endpoint_name)
                    current=true
                >
                    {move || {
                        let synopsis = synopsis.get();
                        view! { <DisplayNode node=synopsis /> }
                    }}
                </BreadcrumbsItem>
            </Breadcrumbs>
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
                                .map(move |param| {
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
                                                    move || {
                                                        let options = options.get();
                                                        match param.get_requirement() {
                                                            node::Cardinality::Required => {
                                                                view! {
                                                                    <FormattedMessage id="engine.cardinality.required" />
                                                                }
                                                                    .into_any()
                                                            }
                                                            node::Cardinality::Optional => {
                                                                view! {
                                                                    <FormattedMessage id="engine.cardinality.optional" />
                                                                }
                                                                    .into_any()
                                                            }
                                                            node::Cardinality::OptionalWithDefault { default } => {
                                                                let mut keys = HashMap::new();
                                                                keys.insert(
                                                                    "default".to_string(),
                                                                    default
                                                                        .serialize(options)
                                                                        .unwrap_or_else(|_| String::from("Serialization failed")),
                                                                );
                                                                view! {
                                                                    <FormattedMessage
                                                                        id="engine.cardinality.default"
                                                                        keys=Signal::derive(move || keys.clone())
                                                                    />
                                                                }
                                                                    .into_any()
                                                            }
                                                            node::Cardinality::Multiple { min, max } => {
                                                                let mut keys = HashMap::new();
                                                                keys.insert("min".to_string(), min.to_string());
                                                                let id = match max {
                                                                    Some(max) => {
                                                                        keys.insert("max".to_string(), max.to_string());
                                                                        "engine.cardinality.range"
                                                                    }
                                                                    None => "engine.cardinality.range.open",
                                                                };
                                                                view! {
                                                                    <FormattedMessage
                                                                        id=id
                                                                        keys=Signal::derive(move || keys.clone())
                                                                    />
                                                                }
                                                                    .into_any()
                                                            }
                                                        }
                                                    }
                                                }
                                            </td>
                                            <td>{move || param.get_description(language.get())}</td>
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
