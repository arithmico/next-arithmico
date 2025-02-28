use common::Language;
use engine::DocumentationModule;
use leptos::prelude::*;

#[component]
pub fn ReferenceSection(module: DocumentationModule) -> impl IntoView {
    view! {
        <details open class="reference-section">
            <summary>{module.name(&Language::English).cloned()}</summary>
            <dl>
                {module
                    .items()
                    .iter()
                    .map(|item| {
                        view! {
                            <>
                                <dt>{item.synopsis(&Language::English).cloned()}</dt>
                                <dd>{item.description(&Language::English).cloned()}</dd>
                            </>
                        }
                    })
                    .collect_view()}
            </dl>
        </details>
    }
}
