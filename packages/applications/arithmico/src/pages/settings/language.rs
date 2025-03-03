use common::Language;
use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    form::listbox::{use_listbox_is_open, Listbox, ListboxDefinition},
    icon::menu_icon::MenuIcon,
};
use web_state::WebState;

use crate::state::{SetLanguageAction, State};

#[component]
pub fn LanguageSetting() -> impl IntoView {
    let state = State::expect_state();
    let language = state.select(|state| state.settings.language);
    let on_change: Callback<Language> =
        Callback::new(move |language: Language| {
            state.dispatch(&SetLanguageAction::new(language));
        });

    let listbox_definition = ListboxDefinition::new()
    .button(move || {
        let is_open = use_listbox_is_open();

        view! {
            <>
                {move || match language.get() {
                    Language::German => {
                        view! { <FormattedMessage id="settings.language.german" /> }
                            .into_any()
                    }
                    Language::English => {
                        view! {
                            <FormattedMessage id="settings.language.english" />
                        }
                            .into_any()
                    }
                }}
                <MenuIcon class=Signal::derive(move || {
                    format!(
                        "icon-hover {}",
                        if is_open.get() { "rotate-0" } else { "rotate-180" },
                    )
                }) />
            </>
        }
    })
    .option(|option| {
        option
            .value(Language::English)
            .view(|| {
                view! { <FormattedMessage id="settings.language.english" /> }
            })
    })
    .option(|option| {
        option
            .value(Language::German)
            .view(|| {
                view! { <FormattedMessage id="settings.language.german" /> }
            })
    });

    view! {
        <div class="flex items-center">
            <span>
                <FormattedMessage id="settings.language" />
            </span>

            <Listbox
                on_change=on_change
                definition=listbox_definition
                value=language
            />

        </div>
    }
}
