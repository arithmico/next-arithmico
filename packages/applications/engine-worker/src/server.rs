use std::{
    ops::DerefMut,
    sync::{Arc, RwLock},
};

use engine::Documentation;
use engine_protocol::{Command, CommandEnvelope, Event};
use web_sys::{
    DedicatedWorkerGlobalScope, MessageEvent, console,
    wasm_bindgen::{JsCast, closure::Closure},
};

use crate::{channel::Channel, state::State};

pub struct Server {
    state: Arc<RwLock<State>>,
    scope: DedicatedWorkerGlobalScope,
}

impl Server {
    pub fn new(scope: DedicatedWorkerGlobalScope) -> Self {
        Self {
            state: Arc::new(RwLock::new(State::new(Channel::new(
                scope.clone(),
            )))),
            scope,
        }
    }

    pub fn set_handler(&self, handler: impl Fn(&Command, &mut State)) {
        let state = self.state.clone();

        match self.state.read() {
            Ok(state) => {
                let documentation = Documentation::from(&state.host_api);
                state.channel.send_event(Event::Ready {
                    documentation: documentation.modules,
                });
            }
            Err(err) => {
                console::error_1(
                    &format!("Failed to read state: {}", err).into(),
                );
                return;
            }
        }

        let on_message = Closure::wrap(Box::new(move |event: MessageEvent| {
            let Some(data) = event.data().as_string() else {
                console::warn_1(&"Received empty message event".into());
                return;
            };
            let command: CommandEnvelope = match serde_json::from_str(&data) {
                Ok(command) => command,
                Err(err) => {
                    console::warn_1(
                        &format!("Failed to deserialize command: {}", err)
                            .into(),
                    );
                    return;
                }
            };

            let mut state = match state.write() {
                Ok(write) => write,
                Err(err) => {
                    console::error_1(
                        &format!("Failed to get state: {}", err).into(),
                    );
                    return;
                }
            };

            state
                .channel
                .set_correlation_id(Some(command.correlation_id));

            handler(&command.payload, state.deref_mut());

            state.channel.set_correlation_id(None);
        }) as Box<dyn Fn(MessageEvent)>);
        self.scope
            .set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();
    }
}
