use common::Language;
use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    form::listbox::{use_listbox_is_open, Listbox, ListboxDefinition},
    icon::menu_icon::MenuIcon,
};

use crate::{
    state::AppAction,
    utils::{expect_dispatch, use_app_state},
};

#[component]
pub fn LanguageSetting() -> impl IntoView {
    let app_state = use_app_state();
    let dispatch = expect_dispatch();

    let on_change: Callback<Language> =
        Callback::new(move |language: Language| {
            dispatch.run(AppAction::SetLanguage(language))
        });

    let listbox_definition = ListboxDefinition::new()
    .button(|button| {
        button
            .view(move || {
                let is_open = use_listbox_is_open();

                view! {
                    <>
                        {move || match app_state.get().settings.language {
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

    })
    .options(|options| {
        options
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
                value=Signal::derive(move || app_state.get().settings.language)
            />

        </div>
    }
}
