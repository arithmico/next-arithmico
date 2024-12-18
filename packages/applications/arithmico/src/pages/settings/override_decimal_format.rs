use common::DecimalFormat;
use leptos::prelude::*;
use translate::FormattedMessage;

use crate::{
    components::{Listbox, ListboxButton, ListboxOption, ListboxOptions},
    state::{override_decimal_format::OverrideDecimalFormat, AppAction},
    utils::{expect_dispatch, use_app_state},
};

#[component]
pub fn OverrideDecimalFormatSetting() -> impl IntoView {
    let app_state = use_app_state();
    let dispatch = expect_dispatch();

    view! {
        <div class="flex items-center">
            <span>
                <FormattedMessage id="settings.override_decimal_format" />
            </span>

            <Listbox
                value=move || {
                    app_state.get().settings.override_decimal_format
                }
                on_change=move |override_decimal_format| {
                    dispatch
                        .call(
                            AppAction::SetOverrideDecimalFormat(override_decimal_format),
                        )
                }
            >
                <ListboxButton>
                    {move || {
                        match app_state
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
                        }
                    }}
                </ListboxButton>

                <ListboxOptions>
                    <ListboxOption value=OverrideDecimalFormat::new()>
                        <FormattedMessage id="settings.override_decimal_format.no" />
                    </ListboxOption>

                    <ListboxOption value=OverrideDecimalFormat::from(
                        DecimalFormat::Comma,
                    )>
                        <FormattedMessage id="settings.language.german" />
                    </ListboxOption>

                    <ListboxOption value=OverrideDecimalFormat::from(
                        DecimalFormat::Dot,
                    )>
                        <FormattedMessage id="settings.language.english" />
                    </ListboxOption>
                </ListboxOptions>
            </Listbox>
        </div>
    }
}
