use leptos::prelude::*;
use translate::{FormattedMessage, IntoTranslationId};

#[component]
pub fn SettingsSection(
    children: Children,
    title: impl IntoTranslationId + 'static,
) -> impl IntoView {
    let title_id = title.into_translation_id().to_string();

    view! {
        <section class="settings-section">
            <h2 class="settings-section-heading">
                <FormattedMessage id=title_id />
            </h2>
            <div class="settings-section-content">{children()}</div>
        </section>
    }
}
