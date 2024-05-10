use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, Callback, Event, Html, Properties,
};

#[derive(PartialEq, Properties)]
pub struct CalculatorFormProps {}

#[function_component]
pub fn CalculatorForm(_props: &CalculatorFormProps) -> Html {
    let onchange = Callback::from(|event: Event| {
        let target = event.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            log!(input.value())
        }
    });

    html! {
        <div class={classes!("flex", "flex-col", "items-center")}>
            <div class={classes!("py-8", "flex", "flex-col", "w-3/5")}>
                <label class={classes!("flex", "flex-col", "w-full")}>
                    {"Eingabe"}
                    <input
                        type={"text"}
                        onchange={onchange}
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
    }
}
