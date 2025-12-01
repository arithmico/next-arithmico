use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    form::listbox::{Listbox, ListboxDefinition, use_listbox_is_open},
    icon::menu_icon::MenuIcon,
};
use web_state::WebState;

use crate::state::{SetThemeAction, State, theme::Theme};

#[component]
pub fn ThemeSetting() -> impl IntoView {
    let state = State::expect_state();
    let theme = state.select(|state| state.settings.theme);
    let on_change = Callback::new(move |theme: Theme| {
        state.dispatch(&SetThemeAction::new(theme));
    });

    let listbox_definition = ListboxDefinition::new()
        .button(move || {
            let is_open = use_listbox_is_open();

            view! {
                <>
                    {move || {
                        let theme = theme.get();
                        view! { <FormattedMessage id=theme /> }
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
        .label(|| view! { <FormattedMessage id="settings.theme" /> })
        .option(Theme::Default, move || view! { <FormattedMessage id=Theme::System /> })
        .option(Theme::Light, move || view! { <FormattedMessage id=Theme::Light /> })
        .option(Theme::Dark, move || view! { <FormattedMessage id=Theme::Dark /> });

    view! {
        <Listbox
            definition=listbox_definition
            value=theme
            on_change=on_change
        />
    }
}
