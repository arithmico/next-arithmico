use crate::{
    components::*,
    state::AppAction,
    utils::{expect_app_state, expect_dispatch},
};
use common::Language;
use common_ui::form::listbox::{
    Listbox, ListboxButton, ListboxOption, ListboxOptions,
};
use leptos::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let app_state = expect_app_state();
    let dispatch = expect_dispatch();

    let class = |selected: bool| {
        let class = "px-2 py-1 hover:bg-neutral-400 rounded-sm focus-visible:outline-2 outline-black";
        format!(
            "{} {}",
            class,
            if selected { "font-bold" } else { "font-normal" }
        )
    };

    view! {
        <PageWithSidebar>
            <PageTitle>Settings</PageTitle>
            <div class="p-2 w-1/2 bg-white rounded-md border border-neutral-200">

                <div class="flex items-center">
                    <span>Language</span>

                    <Listbox
                        class="flex relative flex-col ml-auto w-32"
                        value=move || app_state.get().settings.language
                        on_change=move |language: Language| {
                            dispatch.call(AppAction::SetLanguage(language))
                        }
                    >
                        <ListboxButton class="py-1 px-2 text-left rounded-sm bg-neutral-300 hover:bg-neutral-400">
                            {move || format!("{:?}", app_state.get().settings.language)}
                        </ListboxButton>
                        <ListboxOptions class="absolute mt-1 w-full bg-neutral-300">
                            <ListboxOption class=class value=Language::German>
                                German
                            </ListboxOption>
                            <ListboxOption class=class value=Language::English>
                                English
                            </ListboxOption>
                        </ListboxOptions>
                    </Listbox>
                </div>
            </div>
        </PageWithSidebar>
    }
}
