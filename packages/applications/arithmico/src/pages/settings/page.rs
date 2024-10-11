use crate::components::*;
use common::Language;
use common_ui::form::listbox::{
    Listbox, ListboxButton, ListboxOption, ListboxOptions,
};
use leptos::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let (lang, set_lang) = create_signal(Language::English);

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
            <div class="bg-white rounded-md p-2 w-1/2 border border-neutral-200">

                <div class="flex items-center">
                    <span>Language</span>

                    <Listbox
                        class="flex flex-col relative w-32 ml-auto"
                        value=lang
                        on_change=move |v: Language| set_lang.set(v)
                    >
                        <ListboxButton class="bg-neutral-300 hover:bg-neutral-400 rounded-sm text-left py-1 px-2">
                            {move || format!("{:?}", lang.get())}
                        </ListboxButton>
                        <ListboxOptions class="bg-neutral-300 absolute mt-1 w-full">
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
