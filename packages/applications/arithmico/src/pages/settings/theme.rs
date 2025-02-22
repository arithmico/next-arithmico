use leptos::prelude::*;
use translate::FormattedMessage;
use ui::form::listbox::{use_listbox_is_open, Listbox, ListboxDefinition};

use crate::{
    components::MenuIcon,
    state::{theme::Theme, AppAction},
    utils::{expect_dispatch, use_app_state},
};

#[component]
pub fn ThemeSetting() -> impl IntoView {
    let app_state = use_app_state();
    let dispatch = expect_dispatch();

    let on_change = Callback::new(move |theme: Theme| {
        dispatch.run(AppAction::SetTheme(theme))
    });

    let listbox_definition = ListboxDefinition::new()
        .button(|button| {
            button.view(move || {
                let is_open = use_listbox_is_open();

                view! {
                    <>
                        {move || {
                            let theme = app_state.get().settings.theme;
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
        })
        .options(|options| {
            options
                .option(|option| {
                    option.value(Theme::Light).view(
                        move || view! { <FormattedMessage id=Theme::Light /> },
                    )
                })
                .option(|option| {
                    option.value(Theme::Dark).view(
                        move || view! { <FormattedMessage id=Theme::Dark /> },
                    )
                })
        });

    view! {
        <div class="flex items-center">
            <span>
                <FormattedMessage id="settings.theme" />
            </span>

            <Listbox
                definition=listbox_definition
                value=Signal::derive(move || app_state.get().settings.theme)
                on_change=on_change
            />

        </div>
    }
}
