mod language;
mod override_decimal_format;
mod theme;

use crate::{
    class_names,
    components::*,
    pages::settings::{
        language::LanguageSetting,
        override_decimal_format::OverrideDecimalFormatSetting,
        theme::ThemeSetting,
    },
};

use leptos::*;
use translate::FormattedMessage;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <PageWithSidebar>
            <PageTitle>
                <FormattedMessage id="settings.title" />
            </PageTitle>
            <Card class=class_names!("flex", "flex-col", "gap-2", "p-2")>
                <LanguageSetting />

                <OverrideDecimalFormatSetting />

                <ThemeSetting />
            </Card>
        </PageWithSidebar>
    }
}
