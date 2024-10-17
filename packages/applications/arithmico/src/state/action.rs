use common::Language;

use super::settings::OverrideDecimalFormat;

#[derive(Clone)]
pub enum AppAction {
    Evaluate(String),
    SetLanguage(Language),
    SetOverrideDecimalFormat(OverrideDecimalFormat),
}
