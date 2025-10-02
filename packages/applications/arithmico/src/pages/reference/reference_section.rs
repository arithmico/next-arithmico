use engine::DocumentationModule;
use engine::Language;
use leptos::prelude::*;

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
                                <dt>{item.typed_synopsis(&Language::English).cloned()}</dt>
                                <dd>{item.description(&Language::English).cloned()}</dd>
                            </>
                        }
                    })
                    .collect_view()}
            </dl>
        </section>
    }
}
