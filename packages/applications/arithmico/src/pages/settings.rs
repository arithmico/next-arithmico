use decimal_places::DecimalPlacesSetting;
use leptos::prelude::*;
use settings_section::SettingsSection;
use translate::FormattedMessage;
use ui::common::page_title::PageTitle;

use crate::{
    components::*,
    pages::settings::{
        language::LanguageSetting,
        override_decimal_format::OverrideDecimalFormatSetting,
        theme::ThemeSetting,
    },
};

mod decimal_places;
mod language;
mod override_decimal_format;
mod settings_section;
mod theme;

#[component]
pub fn SettingsPage() -> impl IntoView {
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
            </SettingsSection>
        </PageWithSidebar>
    }
}
