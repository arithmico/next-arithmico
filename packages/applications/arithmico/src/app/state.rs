use js_sys::{
    Array,
    wasm_bindgen::{JsCast, prelude::Closure},
};
use leptos::{
    logging::{error, log},
    prelude::*,
};
use web_state::StateProvider as WebStateProvider;
use web_sys::{Blob, BlobPropertyBag, MessageEvent, Url, Worker};

use crate::state::State;

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

    let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
        if let Some(data) = msg.data().as_string() {
            log!("{}", data);
        }
    }) as Box<dyn Fn(MessageEvent)>);

    worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    let state = RwSignal::new(State::new());

    view! { <WebStateProvider state=state>{children()}</WebStateProvider> }
        .into_any()
}
