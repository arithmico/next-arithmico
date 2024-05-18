use engine::Language;
use yew::prelude::*;

use crate::components::{
    Listbox, ListboxButton, ListboxOption, ListboxOptions,
};

use icons::expand::ExpandIcon;

#[derive(PartialEq, Properties)]
pub struct InterfaceLanguageSelectorProps {}

#[function_component]
pub fn InterfaceLanguageSelector(
    _props: &InterfaceLanguageSelectorProps,
) -> Html {
    let language = use_state(|| Language::English);
    let onchange_language = {
        let language = language.clone();
        Callback::from(move |value: Language| language.set(value))
    };

    html! {
        <label class={classes!("flex", "items-center", "text-xl")}>
            {"Sprache"}
            <Listbox<Language>
                on_change={onchange_language}
                value={(*language).clone()}
                class={classes!("ml-auto")}
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
        </label>
    }
}
