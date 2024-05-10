use yew::{classes, function_component, html, Html, Properties};

use crate::components::PageWithNavbar;

#[derive(PartialEq, Properties)]
pub struct CalculatorPageProps {}

#[function_component]
pub fn CalculatorPage(_props: &CalculatorPageProps) -> Html {
    html! {
        <PageWithNavbar>
            <div class={classes!("flex", "flex-col", "items-center")}>
                <div class={classes!("py-8", "flex", "flex-col", "w-3/5")}>
                    <label class={classes!("flex", "flex-col", "w-full")}>
                        {"Eingabe"}
                        <input
                            type={"text"}
                            class={classes!("border", "border-black", "w-full", "outline-none", "p-2")}
                        />
                    </label>
                    <label class={classes!("flex", "flex-col", "mt-4", "w-full")}>
                        {"Ausgabe"}
                        <input
                            type={"text"}
                            readonly={true}
                            class={classes!("border", "border-black", "w-full", "outline-none", "p-2")}
                        />
                    </label>
                </div>
            </div>
        </PageWithNavbar>
    }
}
