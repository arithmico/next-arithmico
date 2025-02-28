use leptos::prelude::*;
use translate::{use_translate, FormattedMessage};
use ui::{common::page_title::PageTitle, container::page_header::PageHeader};

#[component]
pub fn ReferenceHeader() -> impl IntoView {
    let translate = use_translate();

    view! {
        <PageHeader>
            <PageTitle>
                <FormattedMessage id="reference.title" />
            </PageTitle>
            <input
                type="search"
                class="reference-search"
                placeholder=translate("reference.search", None)
                    .unwrap_or(String::from("TranslationError"))
            />
        </PageHeader>
    }
}
