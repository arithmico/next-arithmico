use common::Language;

use super::{
    settings::override_decimal_format::OverrideDecimalFormat, theme::Theme,
};

#[derive(Clone)]
pub enum AppAction {
    Evaluate(String),
    SetLanguage(Language),
    SetOverrideDecimalFormat(OverrideDecimalFormat),
    SetTheme(Theme),
}
