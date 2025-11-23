use engine::DocumentationModule;
use engine::Language;
use leptos::prelude::*;

use crate::pages::reference::reference_item::ReferenceItem;

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
                            <ReferenceItem
                                item=item.clone()
                                language=Language::English
                            />
                        }
                    })
                    .collect_view()}
            </dl>
        </section>
    }
}
