use leptos::*;
use translate::FormattedMessage;

use crate::{
    class_names,
    components::{Listbox, ListboxButton, ListboxOption, ListboxOptions},
    state::{theme::Theme, AppAction},
    utils::{expect_dispatch, use_app_state},
};

#[component]
pub fn ThemeSetting() -> impl IntoView {
    let app_state = use_app_state();
    let dispatch = expect_dispatch();

    view! {
        <div class="flex items-center">
            <span>
                <FormattedMessage id="settings.theme" />
            </span>

            <Listbox
                class=class_names!(
                    "flex", "relative", "flex-col", "ml-auto", "w-32"
                )
                value=move || app_state.get().settings.theme
                on_change=move |theme| {
                    dispatch.call(AppAction::SetTheme(theme))
                }
            >
                <ListboxButton>
                    {move || {
                        view! {
                            <FormattedMessage id=app_state.get().settings.theme />
                        }
                    }}
                </ListboxButton>

                <ListboxOptions>
                    <ListboxOption value=Theme::Light>
                        <FormattedMessage id=Theme::Light />
                    </ListboxOption>
                    <ListboxOption value=Theme::Dark>
                        <FormattedMessage id=Theme::Dark />
                    </ListboxOption>

                </ListboxOptions>
            </Listbox>
        </div>
    }
}
