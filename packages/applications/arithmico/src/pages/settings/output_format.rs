use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    form::listbox::{use_listbox_is_open, Listbox, ListboxDefinition},
    icon::menu_icon::MenuIcon,
};
use web_state::WebState;

use crate::state::{output_format::OutputFormat, SetOutputFormatAction, State};

#[component]
pub fn OutputFormatSetting() -> impl IntoView {
    let state = State::expect_state();
    let output_format = state.select(|state| state.settings.output_format);
    let on_change: Callback<OutputFormat> =
        Callback::new(move |output_format| {
            state.dispatch(&SetOutputFormatAction::new(output_format));
        });

    let listbox_definition = ListboxDefinition::new()
        .button(move || {
            let is_open = use_listbox_is_open();

            view! {
                <>
                    {move || {
                        let output_format = output_format.get();
                        view! { <FormattedMessage id=output_format /> }
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
        .label(|| view! { <FormattedMessage id="settings.output_format" /> })
        .option(OutputFormat::Text, move || view! { <FormattedMessage id="settings.output_format.text" /> })
        .option(OutputFormat::MathMl, move || view! { <FormattedMessage id="settings.output_format.mathml" /> });

    view! {
        <Listbox
            on_change=on_change
            definition=listbox_definition
            value=output_format
        />
    }
}
