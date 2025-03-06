use editor::transform::{
    MergeTextNodesTransform, RemoveEmptyContainerNodesTransform,
};
use editor_core::EditorState;
use engine::{Session, SessionError};
use trace::Trace;
use web_state::WebState;

use super::Settings;

#[derive(Clone)]
pub struct State {
    pub session: Session,
    pub settings: Settings,
    pub current_output: Option<Result<String, SessionError>>,
    pub current_error_trace: Option<Trace>,
    pub input_editor_state: EditorState,
}

impl State {
    pub fn new() -> Self {
        let mut editor_state = EditorState::new();
        editor_state.add_transform(MergeTextNodesTransform::new());
        editor_state.add_transform(RemoveEmptyContainerNodesTransform::new());

        Self {
            session: Session::new(),
            settings: Settings::load(),
            current_output: None,
            current_error_trace: None,
            input_editor_state: editor_state,
        }
    }
}

impl WebState for State {}
