use crate::{components::*, utils::use_app_state};
use common::Language;
use leptos::*;

#[component]
pub fn ReferencePage() -> impl IntoView {
    let app_state = use_app_state();

    view! {
        <PageWithSidebar>
            <PageTitle>Reference</PageTitle>
            {move || {
                app_state
                    .get()
                    .session
                    .documentation()
                    .modules()
                    .iter()
                    .map(|module| {
                        view! {
                            <details>
                                <summary>{module.name(&Language::English)}</summary>
                                <dl class="grid grid-cols-[1fr_3fr]">
                                    {module
                                        .items()
                                        .iter()
                                        .map(|item| {
                                            view! {
                                                <>
                                                    <dt>{item.synopsis(&Language::English)}</dt>
                                                    <dd>{item.description(&Language::English)}</dd>
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
