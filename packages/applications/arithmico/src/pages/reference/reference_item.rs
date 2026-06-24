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
