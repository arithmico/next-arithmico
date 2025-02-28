use common::DecimalFormat;
use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{form::listbox::{use_listbox_is_open, Listbox, ListboxDefinition}, icon::menu_icon::MenuIcon};

use crate::{
    state::{AppAction, override_decimal_format::OverrideDecimalFormat},
    utils::{expect_dispatch, use_app_state},
};

#[component]
pub fn OverrideDecimalFormatSetting() -> impl IntoView {
    let app_state = use_app_state();
    let dispatch = expect_dispatch();

    let on_change = Callback::new(move |override_decimal_format| {
        dispatch
            .run(AppAction::SetOverrideDecimalFormat(override_decimal_format))
    });

    let listbox_definition = ListboxDefinition::new()
    .button(move || {
        let is_open = use_listbox_is_open();

        view! {
            <>
                {move || match app_state
                    .get()
                    .settings
                    .override_decimal_format
                    .decimal_format()
                {
                    Some(override_format) => {
                        match override_format {
                            DecimalFormat::Comma => {
                                view! { <FormattedMessage id="settings.language.german" /> }
                            }
                            DecimalFormat::Dot => {
                                view! {
                                    <FormattedMessage id="settings.language.english" />
                                }
                            }
                        }
                    }
                    None => {
                        view! {
                            <FormattedMessage id="settings.override_decimal_format.no" />
                        }
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
    .option(|option| 
        option
            .value(OverrideDecimalFormat::new())
            .view(move || view! { <FormattedMessage id="settings.override_decimal_format.no" /> }))
    .option(|option| 
        option
            .value(OverrideDecimalFormat::from(  DecimalFormat::Comma))
            .view(move || view! { <FormattedMessage id="settings.language.german" /> }))
    .option(|option| 
        option
            .value(OverrideDecimalFormat::from(  DecimalFormat::Dot))
            .view(move || view! { <FormattedMessage id="settings.language.english" /> }));

    view! {
        <div class="flex items-center">
            <span>
                <FormattedMessage id="settings.override_decimal_format" />
            </span>

            <Listbox
                value=Signal::derive(move || {
                    app_state.get().settings.override_decimal_format
                })
                on_change=on_change
                definition=listbox_definition
            />

        </div>
    }
}
