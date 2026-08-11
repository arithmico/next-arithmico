use engine::DecimalPlaces;
use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    form::listbox::{Listbox, ListboxDefinition, use_listbox_is_open},
    icon::menu_icon::MenuIcon,
};
use web_state::WebState;

use crate::state::{SetDecimalPlacesAction, State};

#[component]
pub fn DecimalPlacesSetting() -> impl IntoView {
    let state = State::expect_state();
    let mut definition =
        ListboxDefinition::<DecimalPlaces>::new().button(move || {
            let is_open = use_listbox_is_open();

            view! {
                <>
                    {move || {
                        let decimal_places: u8 = (state
                            .get()
                            .settings
                            .decimal_places)
                            .into();
                        decimal_places
                    }}
                    <MenuIcon class=Signal::derive(move || {
                        format!(
                            "icon-hover {}",
                            if is_open.get() { "rotate-0" } else { "rotate-180" },
                        )
                    }) />
                </>
            }
        }).label(|| {
            view! { <FormattedMessage id="settings.decimal_places" /> }
        });

    for i in 0..15u8 {
        definition =
            definition.option(DecimalPlaces::new(i), move || view! { <>{i}</> })
    }

    let on_change: Callback<DecimalPlaces> = Callback::new(move |value| {
        state.dispatch(&SetDecimalPlacesAction::new(value));
    });

    view! {
        <Listbox
            class="decimal-places-setting"
            definition=definition
            value=state.select(|state| state.settings.decimal_places)
            on_change=on_change
        />
    }
}
