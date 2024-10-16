use common::Language;

#[derive(Clone)]
pub enum AppAction {
    Evaluate(String),
    SetLanguage(Language),
}
