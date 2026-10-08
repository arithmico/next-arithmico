use common::NumberRepresentation;
use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    form::listbox::{Listbox, ListboxDefinition, use_listbox_is_open},
    icon::menu_icon::MenuIcon,
};
use web_state::WebState;

use crate::state::{SetNumberRepresentationAction, State};

#[component]
pub fn NumberRepresentationSetting() -> impl IntoView {
    let state = State::expect_state();
    let number_representation =
        state.select(|state| state.settings.number_representation);
    let on_change =
        Callback::new(move |number_representation: NumberRepresentation| {
            state.dispatch(&SetNumberRepresentationAction::new(
                number_representation,
            ));
        });

    let listbox_definition = ListboxDefinition::new()
        .button(move || {
            let is_open = use_listbox_is_open();

            view! {
                <>
                    {move || {
                        let number_representation = number_representation.get();
                        view! {
                            <FormattedMessage id=match number_representation {
                                NumberRepresentation::Number => "settings.number.float",
                                NumberRepresentation::Fraction => "settings.number.fraction",
                                NumberRepresentation::MixedFraction => {
                                    "settings.number.mixed_fraction"
                                }
                            } />
                        }
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
        .label(|| view! { <FormattedMessage id="settings.number" /> })
        .option(NumberRepresentation::Number, move || view! { <FormattedMessage id="settings.number.float" /> })
        .option(NumberRepresentation::Fraction, move || view! { <FormattedMessage id="settings.number.fraction" /> })
        .option(NumberRepresentation::MixedFraction, move || view! { <FormattedMessage id="settings.number.mixed_fraction" /> });

    view! {
        <Listbox
            definition=listbox_definition
            value=number_representation
            on_change=on_change
        />
    }
}
