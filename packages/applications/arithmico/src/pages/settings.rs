use leptos::prelude::*;
use translate::FormattedMessage;
use ui::container::card::Card;

use crate::{
    components::*,
    pages::settings::{
        language::LanguageSetting,
        override_decimal_format::OverrideDecimalFormatSetting,
        theme::ThemeSetting,
    },
};

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
            </Card>
        </PageWithSidebar>
    }
}
