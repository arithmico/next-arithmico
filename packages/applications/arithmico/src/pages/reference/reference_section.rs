use leptos::prelude::*;

use crate::pages::reference::{
    matching::ReferenceModuleMatch, reference_item::ReferenceItem,
};

#[component]
pub fn ReferenceSection(module: ReferenceModuleMatch) -> impl IntoView {
    view! {
        <section class="reference-section">
            <h2>{module.name}</h2>
            <ul>
                {module
                    .matches
                    .into_iter()
                    .map(|item| {
                        view! {
                            <ReferenceItem
                                synopsis=item.synopsis
                                description=item.description
                                url=item.url
                            />
                        }
                    })
                    .collect_view()}
            </ul>
        </section>
    }
}
