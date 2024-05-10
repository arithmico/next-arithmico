use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_context, Callback, Event, Html,
    Properties,
};

use super::CalculatorContext;

#[derive(PartialEq, Properties)]
pub struct CalculatorFormProps {}

#[function_component]
pub fn CalculatorForm(_props: &CalculatorFormProps) -> Html {
    let context = use_context::<CalculatorContext>().unwrap();
    let last_statement = context
        .session()
        .last_statement()
        .and_then(|statement| Some(statement.clone()));

    let onchange = Callback::from(move |event: Event| {
        let target = event.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            context.dispatch(input.value());
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
                        value={match last_statement {
                            Some(statement) => {
                                match &statement.output {
                                    Ok(value) => value.clone(),
                                    Err(error) => error.clone().to_string(),
                                }
                            },
                            None => "".to_string(),
                        }}
                        class={classes!("border", "border-black", "w-full", "outline-none", "p-2")}
                    />
                </label>
            </div>
        </div>
    }
}
