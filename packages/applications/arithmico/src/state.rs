use std::collections::BTreeMap;

pub use actions::*;
use common::Language;
use editor::transform::{
    MergeTextNodesTransform, RemoveEmptyContainerNodesTransform,
};
use editor_core::EditorState;
use engine::{Documentation, SerializeOptions};
use engine_protocol::Error;
use node::Node;
use trace::Span;

mod actions;
mod log;
mod settings;
mod worker_client;

pub use log::*;
pub use settings::*;
use web_sys::Worker;

use crate::state::worker_client::WorkerClient;

pub struct State {
    pub log: Log,
    pub symbols: BTreeMap<String, Node>,
    pub documentation: Documentation,
    pub settings: Settings,
    pub current_output: Option<Result<Node, Error>>,
    pub error_spans: Vec<Span>,
    pub input_editor_state: EditorState,
    pub worker_client: WorkerClient,
}

impl State {
    pub fn new(worker: Worker) -> Self {
        let mut editor_state = EditorState::default();
        editor_state.add_transform(MergeTextNodesTransform);
        editor_state.add_transform(RemoveEmptyContainerNodesTransform);

        Self {
            log: Log::default(),
            symbols: BTreeMap::new(),
            documentation: Documentation::default(),
            settings: Settings::load(),
            current_output: None,
            error_spans: vec![],
            input_editor_state: editor_state,
            worker_client: WorkerClient::new(worker),
        }
    }

    pub fn create_serialize_options(&self) -> SerializeOptions {
        let decimal_places = self.settings.decimal_places;
        let language = self.get_decimal_format();
        let number_representation = self.settings.number_representation;

        SerializeOptions {
            language,
            decimal_places,
            number_representation,
        }
    }

    pub fn get_decimal_format(&self) -> Language {
        self.settings
            .override_decimal_format
            .decimal_format()
            .unwrap_or_else(|| self.settings.get_language())
    }
}
