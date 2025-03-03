use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    form::listbox::{use_listbox_is_open, Listbox, ListboxDefinition},
    icon::menu_icon::MenuIcon,
};
use web_state::WebState;

use crate::state::{theme::Theme, SetThemeAction, State};

#[component]
pub fn ThemeSetting() -> impl IntoView {
    let state = State::use_state();
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
        .option(|option| {
            option.value(Theme::Light).view(
                move || view! { <FormattedMessage id=Theme::Light /> },
            )
        })
        .option(|option| {
            option.value(Theme::Dark).view(
                move || view! { <FormattedMessage id=Theme::Dark /> },
            )
        });

    view! {
        <div class="flex items-center">
            <span>
                <FormattedMessage id="settings.theme" />
            </span>

            <Listbox
                definition=listbox_definition
                value=theme
                on_change=on_change
            />

        </div>
    }
}
