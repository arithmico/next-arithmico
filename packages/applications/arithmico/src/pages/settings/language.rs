use common::Language;
use leptos::*;
use translate::FormattedMessage;

use crate::{
    components::{Listbox, ListboxButton, ListboxOption, ListboxOptions},
    state::AppAction,
    utils::{expect_dispatch, use_app_state},
};

#[component]
pub fn LanguageSetting() -> impl IntoView {
    let app_state = use_app_state();
    let dispatch = expect_dispatch();

    view! {
        <div class="flex items-center">
            <span>
                <FormattedMessage id="settings.language" />
            </span>

            <Listbox
                value=move || app_state.get().settings.language
                on_change=move |language: Language| {
                    dispatch.call(AppAction::SetLanguage(language))
                }
            >
                <ListboxButton>
                    {move || {
                        view! {
                            <FormattedMessage id=format!(
                                "settings.language.{}",
                                app_state.get().settings.language.to_string().to_lowercase(),
                            ) />
                        }
                    }}

                </ListboxButton>
                <ListboxOptions>
                    <ListboxOption value=Language::German>
                        <FormattedMessage id="settings.language.german" />
                    </ListboxOption>
                    <ListboxOption value=Language::English>
                        <FormattedMessage id="settings.language.english" />
                    </ListboxOption>
                </ListboxOptions>
            </Listbox>
        </div>
    }
}
