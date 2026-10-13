use common::{AngleUnit, Language};
use engine_protocol::{Command, Error, Event};
use evaluator::{EvaluateNode, Options};
use node::Node;
use parser::parse;
use web_sys::{
    DedicatedWorkerGlobalScope, console, js_sys::global, wasm_bindgen::JsValue,
};

use crate::{server::Server, state::State};

mod channel;
mod server;
mod state;

fn main() {
    console_error_panic_hook::set_once();

    let scope = DedicatedWorkerGlobalScope::from(JsValue::from(global()));

    let server = Server::new(scope);
    server.set_handler(handler);

    console::log_1(&"WebWorker is ready!".into());
}

fn handler(command: &Command, state: &mut State) {
    match command {
        Command::Evaluate {
            input,
            language,
            angle_unit,
        } => {
            let output = evaluate(input, *language, *angle_unit, state);
            if let Ok(Node::Definition(node)) = &output {
                state
                    .stack
                    .insert(&node.symbol, node.expression.as_ref().clone());
                state.channel.send_event(Event::StackItemUpdated {
                    name: node.symbol.clone(),
                    value: node.expression.as_ref().clone(),
                });
            }
            state
                .channel
                .send_event(Event::EvaluationComplete { output });
        }
        Command::ClearStack => {
            state.stack.clear();
            state.channel.send_event(Event::StackCleared);
        }
    }
}

fn evaluate(
    input: &str,
    language: Language,
    angle_unit: AngleUnit,
    state: &State,
) -> Result<Node, Error> {
    let node = parse(input, language)?;
    let options = Options::new(&state.stack, &state.host_api, angle_unit);
    let output = node.evaluate(options)?;
    Ok(output)
}
