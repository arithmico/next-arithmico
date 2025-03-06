use decimal_places::DecimalPlacesSetting;
use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{common::page_title::PageTitle, container::card::Card};

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
mod theme;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <PageWithSidebar>
            <PageTitle>
                <FormattedMessage id="settings.title" />
            </PageTitle>
            <Card class="settings-card">
                <LanguageSetting />
                <OverrideDecimalFormatSetting />
                <ThemeSetting />
                <DecimalPlacesSetting />
            </Card>
        </PageWithSidebar>
    }
}
