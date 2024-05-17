use yew::Reducible;

use super::AppState;

pub enum AppReducerAction {
    Evaluate(String),
}

impl Reducible for AppState {
    type Action = AppReducerAction;

    fn reduce(
        self: std::rc::Rc<Self>,
        action: Self::Action,
    ) -> std::rc::Rc<Self> {
        match action {
            AppReducerAction::Evaluate(input) => Self {
                host_api: self.host_api.clone(),
                session: self.session.push(input.as_str()),
            }
            .into(),
        }
    }
}
