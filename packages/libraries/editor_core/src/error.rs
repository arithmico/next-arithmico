use thiserror::Error;
use web_sys::{Element, wasm_bindgen::JsValue};

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("JsValue error")]
    Js(JsValue),

    #[error("Failed to create element")]
    FailedToCreateElement(Element),

    #[error("Missing selection")]
    MissingSelection,

    #[error("Selection collapsed")]
    SelectionCollapsed,

    #[error("Node not found")]
    NodeNotFound,

    #[error("Not a leaf node")]
    NotALeafNode,

    #[error("Not a container node")]
    NotAContainerNode,

    #[error("Node cast failed")]
    NodeCastFailed,

    #[error("Missing DOM node")]
    MissingDomNode,

    #[error("Parent node not found")]
    ParentNodeNotFound,
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Self::Js(value)
    }
}
