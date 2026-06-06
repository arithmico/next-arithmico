use engine::Language;
use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    form::listbox::{Listbox, ListboxDefinition, use_listbox_is_open},
    icon::menu_icon::MenuIcon,
};
use web_state::WebState;

use crate::state::{SetLanguageAction, State, language::LanguageValue};

#[component]
pub fn LanguageSetting() -> impl IntoView {
    let state = State::expect_state();
    let language = state.select(|state| state.settings.language);
    let on_change = Callback::new(move |language: LanguageValue| {
        state.dispatch(&SetLanguageAction::new(language));
    });

    let listbox_definition = ListboxDefinition::new()
    .button(move || {
        let is_open = use_listbox_is_open();

        view! {
            <>
                <span data-testid="language-setting-value">
                    {move || match language.get() {
                        LanguageValue::System => {
                            view! { <FormattedMessage id="settings.language.system" /> }
                                .into_any()
                        }
                        LanguageValue::Language(Language::German) => {
                            view! { <FormattedMessage id="settings.language.german" /> }
                                .into_any()
                        }
                        LanguageValue::Language(Language::English) => {
                            view! {
                                <FormattedMessage id="settings.language.english" />
                            }
                                .into_any()
                        }
                    }}
                </span>
                <MenuIcon class=Signal::derive(move || {
                    format!(
                        "icon-hover {}",
                        if is_open.get() { "rotate-0" } else { "rotate-180" },
                    )
                }) />
            </>
        }
    })
    .label(|| {
        view! { <FormattedMessage id="settings.language" /> }
    })
    .option(LanguageValue::System, || view! { <FormattedMessage id="settings.language.system" /> })
    .option(LanguageValue::Language(Language::English), || view! { <FormattedMessage id="settings.language.english" /> })
    .option(LanguageValue::Language(Language::German), || view! { <FormattedMessage id="settings.language.german" /> });

    view! {
        <Listbox
            on_change=on_change
            definition=listbox_definition
            value=language
        />
    }
}
