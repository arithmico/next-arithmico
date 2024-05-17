use engine::Language;
use headless_components::listbox::{
    Listbox, ListboxButton, ListboxOption, ListboxOptions,
};
use yew::prelude::*;

use crate::components::PageWithNavbar;

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
                class={classes!("relative", "flex", "flex-col", "w-36")}
            >
                <ListboxButton<Language>
                    class={classes!(
                        "bg-neutral-300",
                        "px-2",
                        "py-1",
                        "rounded-sm",
                        "w-full",
                        "items-center",
                        "flex",
                        "group"
                    )}
                >
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
                    <ListboxOptions<Language>
                        class={classes!(
                            "absolute",
                            "bg-neutral-300",
                            "mt-2",
                            "w-full",
                            "rounded-sm",
                            "outline-black",
                            "focus:outline-2",
                            "focus-visible:outline-2"
                        )}
                    >
                        <ListboxOption<Language>
                            value={Language::German}
                            class={classes!(
                                "bg-neutral-300",
                                "px-2",
                                "py-1",
                                "rounded-sm",
                                "w-full",
                                "text-left",
                                "outline-black",
                                "focus:outline-2",
                                "focus-visible:outline-2"
                            )}
                        >
                            {"Deutsch"}
                        </ListboxOption<Language>>
                        <ListboxOption<Language>
                            value={Language::English}
                            class={classes!(
                                "bg-neutral-300",
                                "px-2",
                                "py-1",
                                "rounded-sm",
                                "w-full",
                                "text-left",
                                "outline-black",
                                "focus:outline-2",
                                "focus-visible:outline-2"
                            )}
                        >
                            {"Englisch"}
                        </ListboxOption<Language>>
                    </ListboxOptions<Language>>
                </div>
            </Listbox<Language>>
        </PageWithNavbar>
    }
}
