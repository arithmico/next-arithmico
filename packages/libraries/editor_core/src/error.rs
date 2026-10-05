use thiserror::Error;
use web_sys::wasm_bindgen::JsValue;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("JsError")]
    Js(JsValue),

    #[error("Missing selection")]
    MissingSelection,
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Self::Js(value)
    }
}
