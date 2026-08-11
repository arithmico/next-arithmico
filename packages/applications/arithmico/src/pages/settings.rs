use decimal_places::DecimalPlacesSetting;
use leptos::prelude::*;
use settings_section::SettingsSection;
use translate::FormattedMessage;
use ui::common::page_title::PageTitle;
use web_state::WebState;

use crate::{
    components::*,
    pages::settings::{
        angle_unit::AngleUnitSetting, language::LanguageSetting,
        override_decimal_format::OverrideDecimalFormatSetting,
        theme::ThemeSetting,
    },
    state::{ResetSettingsAction, State},
};

mod angle_unit;
mod decimal_places;
mod language;
mod override_decimal_format;
mod settings_section;
mod theme;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let state = State::expect_state();

    view! {
        <PageWithSidebar class="settings-page">
            <PageTitle>
                <FormattedMessage id="settings.title" />
            </PageTitle>

            <SettingsSection title="settings.section.general">
                <LanguageSetting />
                <ThemeSetting />
            </SettingsSection>

            <SettingsSection title="settings.section.calculator">
                <OverrideDecimalFormatSetting />
                <DecimalPlacesSetting />
                <AngleUnitSetting />
            </SettingsSection>

            <SettingsSection title="settings.section.misc">
                <div class="reset-settings-container">
                    <span>
                        <FormattedMessage id="settings.section.reset-settings" />
                    </span>
                    <button
                        class="reset-settings-button"
                        on:click=move |_| {
                            state.dispatch(&ResetSettingsAction::new());
                        }
                    >
                        <FormattedMessage id="settings.section.reset-settings.reset" />
                    </button>
                </div>
            </SettingsSection>
        </PageWithSidebar>
    }
}
