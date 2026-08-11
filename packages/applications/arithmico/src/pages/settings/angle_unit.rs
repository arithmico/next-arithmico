use engine::AngleUnit;
use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    form::listbox::{Listbox, ListboxDefinition, use_listbox_is_open},
    icon::menu_icon::MenuIcon,
};
use web_state::WebState;

use crate::state::{SetAngleUnitAction, State};

#[component]
pub fn AngleUnitSetting() -> impl IntoView {
    let state = State::expect_state();
    let angle_unit = state.select(|state| state.settings.angle_unit);
    let on_change = Callback::new(move |angle_unit: AngleUnit| {
        state.dispatch(&SetAngleUnitAction::new(angle_unit));
    });

    let listbox_definition = ListboxDefinition::new()
        .button(move || {
            let is_open = use_listbox_is_open();

            view! {
                <>
                    {move || {
                        let angle_unit = angle_unit.get();
                        view! {
                            <FormattedMessage id=match angle_unit {
                                AngleUnit::Radian => "settings.angle_unit.radian",
                                AngleUnit::Degree => "settings.angle_unit.degree",
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
        .label(|| view! { <FormattedMessage id="settings.angle_unit" /> })
        .option(AngleUnit::Radian, move || view! { <FormattedMessage id="settings.angle_unit.radian" /> })
        .option(AngleUnit::Degree, move || view! { <FormattedMessage id="settings.angle_unit.degree" /> });

    view! {
        <Listbox
            definition=listbox_definition
            value=angle_unit
            on_change=on_change
        />
    }
}
