use crate::components::{
    Listbox, ListboxButton, ListboxOption, ListboxOptions, PageWithNavbar,
};
use engine::Language;

use yew::prelude::*;

use icons::ExpandIcon;

#[derive(PartialEq, Properties)]
pub struct SettingsPageProps {}

#[function_component]
pub fn SettingsPage(_props: &SettingsPageProps) -> Html {
    let language = use_state(|| Language::English);
    let onchange_language = {
        let language = language.clone();
        Callback::from(move |value: Language| language.set(value))
    };

    html! {
        <PageWithNavbar>
            <p>{"settings"}</p>
            <Listbox<Language>
                on_change={onchange_language}
                value={(*language).clone()}
            >
                <ListboxButton<Language>>
                    {match *language {
                        Language::German => html!({"Deutsch"}),
                        Language::English => html!({"Englisch"}),
                    }}
                    <ExpandIcon class={classes!(
                        "w-4",
                        "h-4",
                        "ml-auto",
                        "group-data-[expanded=true]:rotate-180"
                    )} />
                </ListboxButton<Language>>
                <div>
                    <ListboxOptions<Language>>
                        <ListboxOption<Language> value={Language::German}>
                            {"Deutsch"}
                        </ListboxOption<Language>>
                        <ListboxOption<Language> value={Language::English}>
                            {"Englisch"}
                        </ListboxOption<Language>>
                    </ListboxOptions<Language>>
                </div>
            </Listbox<Language>>
        </PageWithNavbar>
    }
}
