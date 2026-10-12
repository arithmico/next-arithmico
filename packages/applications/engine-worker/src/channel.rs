use engine_protocol::{Event, EventEnvelope};
use web_sys::{DedicatedWorkerGlobalScope, console, wasm_bindgen::JsValue};

#[derive(Debug, Clone)]
pub struct Channel {
    scope: DedicatedWorkerGlobalScope,
    correlation_id: Option<usize>,
}

impl Channel {
    pub fn new(scope: DedicatedWorkerGlobalScope) -> Self {
        Self {
            scope,
            correlation_id: None,
        }
    }

    pub fn set_correlation_id(&mut self, correlation_id: Option<usize>) {
        self.correlation_id = correlation_id;
    }

    pub fn send_event(&self, event: Event) {
        let message = match serde_json::to_string(&EventEnvelope {
            correlation_id: self.correlation_id,
            payload: event,
        }) {
            Ok(message) => message,
            Err(err) => {
                console::error_1(
                    &format!("Failed to serialize message: {}", err).into(),
                );
                return;
            }
        };
        if let Err(err) = self.scope.post_message(&JsValue::from_str(&message))
        {
            console::error_1(
                &format!("Failed to post message: {:?}", err).into(),
            );
        }
    }
}
