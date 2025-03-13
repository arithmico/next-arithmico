use engine::DecimalFormat;
use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    form::listbox::{Listbox, ListboxDefinition, use_listbox_is_open},
    icon::menu_icon::MenuIcon,
};
use web_state::WebState;

use crate::state::{
    SetOverrideDecimalFormatAction, State,
    override_decimal_format::OverrideDecimalFormat,
};

#[component]
pub fn OverrideDecimalFormatSetting() -> impl IntoView {
    let state = State::expect_state();
    let override_decimal_format =
        state.select(|state| state.settings.override_decimal_format);

    let on_change = Callback::new(move |override_decimal_format| {
        state.dispatch(&SetOverrideDecimalFormatAction::new(
            override_decimal_format,
        ));
    });

    let listbox_definition = ListboxDefinition::new()
    .button(move || {
        let is_open = use_listbox_is_open();

        view! {
            <>
                {move || match override_decimal_format.get().decimal_format() {
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
    .label(|| {
        view! { <FormattedMessage id="settings.override_decimal_format" /> }
    })
    .option(OverrideDecimalFormat::new(), move || view! { <FormattedMessage id="settings.override_decimal_format.no" /> })
    .option(OverrideDecimalFormat::from(DecimalFormat::Comma), move || view! { <FormattedMessage id="settings.language.german" /> })
    .option(OverrideDecimalFormat::from(DecimalFormat::Dot), move || view! { <FormattedMessage id="settings.language.english" /> });

    view! {
        <Listbox
            value=override_decimal_format
            on_change=on_change
            definition=listbox_definition
        />
    }
}
