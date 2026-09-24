use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};
use translate::FormattedMessage;

#[component]
pub fn ReferenceItem(
    synopsis: String,
    description: String,
    url: String,
) -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <li
            class="reference-item"
            on:click=move |_| {
                navigate(&url, Default::default());
            }
        >
            <h3>{synopsis}</h3>
            <div>
                <p>{description}</p>
                <A href=url.clone() {..} class="sr-only">
                    <FormattedMessage id="reference.item.details" />
                </A>
            </div>
        </li>
    }
}
