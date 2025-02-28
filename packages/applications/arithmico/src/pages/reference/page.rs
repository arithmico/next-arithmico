use crate::{components::*, utils::use_app_state};
use common::Language;
use leptos::prelude::*;
use translate::{use_translate, FormattedMessage};
use ui::{common::page_title::PageTitle, container::page_header::PageHeader};

#[component]
pub fn ReferencePage() -> impl IntoView {
    let app_state = use_app_state();
    let translate = use_translate();

    view! {
        <PageWithSidebar>
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
            {move || {
                app_state
                    .get()
                    .session
                    .documentation()
                    .modules()
                    .iter()
                    .map(|module| {
                        view! {
                            <details open class="reference-section">
                                <summary>
                                    {module.name(&Language::English).cloned()}
                                </summary>
                                <dl>
                                    {module
                                        .items()
                                        .iter()
                                        .map(|item| {
                                            view! {
                                                <>
                                                    <dt class="py-2">
                                                        {item.synopsis(&Language::English).cloned()}
                                                    </dt>
                                                    <dd class="py-2">
                                                        {item.description(&Language::English).cloned()}
                                                    </dd>
                                                </>
                                            }
                                        })
                                        .collect_view()}
                                </dl>
                            </details>
                        }
                    })
                    .collect_view()
            }}

        </PageWithSidebar>
    }
}
