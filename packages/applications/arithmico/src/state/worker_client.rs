use engine_protocol::{Command, CommandEnvelope};
use js_sys::wasm_bindgen::JsValue;
use leptos::logging::error;
use web_sys::Worker;

#[derive(Debug, Clone)]
pub struct WorkerClient {
    worker: Worker,
    next_correlation_id: usize,
}

impl WorkerClient {
    pub fn new(worker: Worker) -> Self {
        Self {
            worker,
            next_correlation_id: 0,
        }
    }

    pub fn reserve_correlation_id(&mut self) -> usize {
        let correlation_id = self.next_correlation_id;
        self.next_correlation_id += 1;
        correlation_id
    }

    pub fn send_command(&mut self, correlation_id: usize, command: Command) {
        let message = match serde_json::to_string(&CommandEnvelope {
            correlation_id,
            payload: command,
        }) {
            Ok(message) => message,
            Err(err) => {
                error!("Failed to serialize command: {}", err);
                return;
            }
        };

        if let Err(err) = self.worker.post_message(&JsValue::from_str(&message))
        {
            error!("Failed to send message to worker: {:?}", err);
        }
    }
}
