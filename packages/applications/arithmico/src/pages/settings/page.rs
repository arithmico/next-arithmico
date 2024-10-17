use crate::{
    components::*,
    state::{AppAction, OverrideDecimalFormat},
    utils::{expect_app_state, expect_dispatch},
};
use common::{DecimalFormat, Language};
use common_ui::form::listbox::{
    Listbox, ListboxButton, ListboxOption, ListboxOptions,
};
use leptos::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let app_state = expect_app_state();
    let dispatch = expect_dispatch();

    let class = |selected: bool| {
        let class = "px-2 py-1 hover:bg-neutral-300 rounded-sm focus-visible:outline-2 outline-black";
        format!(
            "{} {}",
            class,
            if selected { "font-bold" } else { "font-normal" }
        )
    };

    view! {
        <PageWithSidebar>
            <PageTitle>Settings</PageTitle>
            <div class="flex flex-col gap-2 p-2 w-1/2 bg-white rounded-md border border-neutral-200">
                <div class="flex items-center">
                    <span>Language</span>

                    <Listbox
                        class="flex relative flex-col ml-auto w-32"
                        value=move || app_state.get().settings.language
                        on_change=move |language: Language| {
                            dispatch.call(AppAction::SetLanguage(language))
                        }
                    >
                        <ListboxButton class="py-1 px-2 text-left rounded-sm border bg-neutral-200 border-neutral-300 hover:bg-neutral-300">
                            {move || format!("{:?}", app_state.get().settings.language)}
                        </ListboxButton>
                        <ListboxOptions class="absolute z-10 mt-1 w-full border bg-neutral-200 border-neutral-300">
                            <ListboxOption class=class value=Language::German>
                                German
                            </ListboxOption>
                            <ListboxOption class=class value=Language::English>
                                English
                            </ListboxOption>
                        </ListboxOptions>
                    </Listbox>
                </div>

                <div class="flex items-center">
                    <span>Override decimal format</span>

                    <Listbox
                        class="flex relative flex-col ml-auto w-32"
                        value=move || app_state.get().settings.override_decimal_format
                        on_change=move |override_decimal_format| {
                            dispatch
                                .call(AppAction::SetOverrideDecimalFormat(override_decimal_format))
                        }
                    >
                        <ListboxButton class="py-1 px-2 text-left rounded-sm border bg-neutral-200 border-neutral-300 hover:bg-neutral-300">
                            {move || {
                                format!("{:?}", app_state.get().settings.override_decimal_format)
                            }}
                        </ListboxButton>
                        <ListboxOptions class="absolute z-10 mt-1 w-full border bg-neutral-200 border-neutral-300">
                            <ListboxOption class=class value=OverrideDecimalFormat::new()>
                                Default
                            </ListboxOption>

                            <ListboxOption
                                class=class
                                value=OverrideDecimalFormat::from(DecimalFormat::Comma)
                            >
                                German
                            </ListboxOption>
                            <ListboxOption
                                class=class
                                value=OverrideDecimalFormat::from(DecimalFormat::Dot)
                            >
                                English
                            </ListboxOption>
                        </ListboxOptions>
                    </Listbox>
                </div>
            </div>
        </PageWithSidebar>
    }
}
