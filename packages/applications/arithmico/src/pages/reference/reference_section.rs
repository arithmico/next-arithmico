use engine::DocumentationModule;
use engine::Language;
use leptos::prelude::*;

use crate::pages::reference::reference_item_information::ReferenceItemInformation;

#[component]
pub fn ReferenceSection(module: DocumentationModule) -> impl IntoView {
    view! {
        <section class="reference-section">
            <h2>{module.name(&Language::English).cloned()}</h2>
            <dl>
                {module
                    .items()
                    .iter()
                    .map(|item| {
                        view! {
                            <>
                                <dt>{item.synopsis(&Language::English).cloned()}</dt>
                                <dd>
                                    <ReferenceItemInformation item=item.clone() />
                                </dd>
                            </>
                        }
                    })
                    .collect_view()}
            </dl>
        </section>
    }
}
