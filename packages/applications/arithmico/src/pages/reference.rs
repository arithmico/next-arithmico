use crate::{components::*, utils::use_app_state};
use common::Language;
use leptos::prelude::*;
use reference_header::ReferenceHeader;

mod reference_header;

#[component]
pub fn ReferencePage() -> impl IntoView {
    let app_state = use_app_state();

    view! {
        <PageWithSidebar>
            <ReferenceHeader />
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
