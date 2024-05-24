use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_context, Callback, Event, Html,
    InputEvent, Properties,
};

use crate::app_context::{AppAction, AppContext};

#[derive(PartialEq, Properties)]
pub struct FormProps {}

#[function_component]
pub fn Form(_props: &FormProps) -> Html {
    let context = use_context::<AppContext>().unwrap();
    let last_statement = context
        .get_last_statement()
        .and_then(|statement| Some(statement.clone()));

    let onchange = {
        let context = context.clone();
        Callback::from(move |_event: Event| {
            context.dispatch(AppAction::Evaluate);
        })
    };

    let oninput = {
        let context = context.clone();
        Callback::from(move |event: InputEvent| {
            let target = event
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            context.dispatch(AppAction::SetInput(target.value()))
        })
    };

    html! {
        <>
            <label class={classes!("flex", "flex-col", "w-full")}>
                <span class={classes!("sr-only")}>{"Eingabe"}</span>
                <input
                    type={"text"}
                    onchange={onchange}
                    oninput={oninput}
                    value={context.input.clone()}
                    placeholder={"Eingabe"}
                    class={classes!(
                        "outline-1",
                        "outline-black",
                        "w-full",
                        "outline-none",
                        "p-4",
                        "rounded-sm",
                        "text-3xl",
                        "focus:outline-2",
                        "bg-neutral-100"
                    )}
                />
            </label>
            <label class={classes!("flex", "flex-col", "mt-12", "w-full")}>
                <span class={classes!("sr-only")}>{"Ausgabe"}</span>
                <input
                    type={"text"}
                    readonly={true}
                    placeholder={"Ausgabe"}
                    value={match last_statement {
                        Some(statement) => {
                            match &statement.output {
                                Ok(value) => value.clone(),
                                Err(error) => error.clone().to_string(),
                            }
                        },
                        None => "".to_string(),
                    }}
                    class={classes!(
                        "outline-1",
                        "outline-black",
                        "w-full",
                        "outline-none",
                        "p-4",
                        "rounded-sm",
                        "text-3xl",
                        "focus:outline-2",
                        "bg-neutral-100"
                    )}
                />
            </label>
        </>
    }
}
