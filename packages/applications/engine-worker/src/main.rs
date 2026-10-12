use std::sync::{Arc, Mutex};

use engine_protocol::{Command, CommandEnvelope, Event, EventEnvelope};
use web_sys::{
    DedicatedWorkerGlobalScope, MessageEvent, console,
    js_sys::global,
    wasm_bindgen::{JsCast, JsValue, closure::Closure},
};

use crate::state::State;

mod state;

fn main() {
    console_error_panic_hook::set_once();

    let _state = Arc::new(Mutex::new(State::default()));

    let scope = DedicatedWorkerGlobalScope::from(JsValue::from(global()));

    let on_message = Closure::wrap(Box::new(move |event: MessageEvent| {
        let Some(data) = event.data().as_string() else {
            console::warn_1(&"Received empty message event".into());
            return;
        };
        let command: CommandEnvelope = match serde_json::from_str(&data) {
            Ok(command) => command,
            Err(err) => {
                console::warn_1(
                    &format!("Failed to deserialize command: {}", err).into(),
                );
                return;
            }
        };

        match command.payload {
            Command::Evaluate { .. } => {
                console::info_1(&"Received evaluate command".into());
            }
            Command::ClearStack => {
                console::info_1(&"Received clear stack command".into());
            }
        }
    }) as Box<dyn Fn(MessageEvent)>);
    scope.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();

    let ready_message = match serde_json::to_string(&EventEnvelope {
        correlation_id: None,
        payload: Event::Ready,
    }) {
        Ok(ready_message) => ready_message,
        Err(err) => {
            console::error_1(
                &format!("Failed to serialize message: {}", err).into(),
            );
            return;
        }
    };

    if let Err(err) = scope.post_message(&JsValue::from_str(&ready_message)) {
        console::error_1(&format!("Failed to send message: {:?}", err).into());
    }

    console::log_1(&"WebWorker is ready!".into());
}
