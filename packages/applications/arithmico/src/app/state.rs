use std::ops::DerefMut;

use engine_protocol::{Event, EventEnvelope};
use js_sys::{
    Array,
    wasm_bindgen::{JsCast, prelude::Closure},
};
use leptos::{
    logging::{error, warn},
    prelude::*,
};
use web_state::{StateMutation, StateProvider as WebStateProvider};
use web_sys::{Blob, BlobPropertyBag, MessageEvent, Url, Worker};

use crate::state::{State, UpdateOutputAction};

fn create_worker(name: &str) -> Option<Worker> {
    let origin = window().location().origin().ok()?;

    let script = Array::new();
    script.push(
        &format!(r#"importScripts("{origin}/{name}.js");wasm_bindgen("{origin}/{name}_bg.wasm");"#)
            .into(),
    );

    let options = BlobPropertyBag::new();
    options.set_type("text/javascript");

    let blob =
        Blob::new_with_str_sequence_and_options(&script, &options).ok()?;

    let url = Url::create_object_url_with_blob(&blob).ok()?;

    Worker::new(&url).ok()
}

#[component]
pub fn StateProvider(children: Children) -> impl IntoView {
    let Some(worker) = create_worker("engine-worker") else {
        error!("Failed to create WebWorker!");
        return ().into_any();
    };

    let state = RwSignal::new(State::new(worker.clone()));

    let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
        let Some(message) = msg.data().as_string() else {
            warn!("Received empty message from worker.");
            return;
        };
        let event = match serde_json::from_str::<EventEnvelope>(&message) {
            Ok(event) => event,
            Err(err) => {
                error!("Failed to deserialize event: {}", err);
                return;
            }
        };
        match event.payload {
            Event::Ready { documentation } => {
                let mut state = state.write();
                state.documentation = documentation;
            }
            Event::EvaluationComplete { output } => {
                let Some(correlation_id) = event.correlation_id else {
                    error!("Missing correlation_id");
                    return;
                };
                let action = UpdateOutputAction::new(correlation_id, output);
                action.apply(state.write().deref_mut());
            }
            Event::StackItemUpdated { name, value } => {
                let mut state = state.write();
                state.symbols.insert(name, value);
            }
            Event::StackCleared => {
                let mut state = state.write();
                state.symbols.clear();
            }
        }
    }) as Box<dyn Fn(MessageEvent)>);

    worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    view! { <WebStateProvider state=state>{children()}</WebStateProvider> }
        .into_any()
}
