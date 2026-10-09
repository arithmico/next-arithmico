use common::Language;
use editor::transform::{
    MergeTextNodesTransform, RemoveEmptyContainerNodesTransform,
};
use editor_core::EditorState;
use engine::{SerializeOptions, Session, SessionError};
use node::Node;
use trace::Span;
use web_state::WebState;

use super::Settings;

#[derive(Clone)]
pub struct State {
    pub session: Session,
    pub settings: Settings,
    pub current_output: Option<Result<Node, SessionError>>,
    pub error_spans: Vec<Span>,
    pub input_editor_state: EditorState,
}

impl State {
    pub fn new() -> Self {
        let mut editor_state = EditorState::default();
        editor_state.add_transform(MergeTextNodesTransform);
        editor_state.add_transform(RemoveEmptyContainerNodesTransform);

        Self {
            session: Session::new(),
            settings: Settings::load(),
            current_output: None,
            error_spans: vec![],
            input_editor_state: editor_state,
        }
    }

    pub fn create_serialize_options(&self) -> SerializeOptions {
        let decimal_places = self.settings.decimal_places;
        let language = self.get_decimal_format();

        SerializeOptions {
            language,
            decimal_places,
        }
    }

    pub fn get_decimal_format(&self) -> Language {
        self.settings
            .override_decimal_format
            .decimal_format()
            .unwrap_or_else(|| self.settings.get_language())
    }
}

impl WebState for State {}
