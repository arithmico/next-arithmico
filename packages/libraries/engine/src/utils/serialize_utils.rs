use crate::{core::context::Context, Language};

pub fn get_decimal_separator(context: &Context) -> String {
    match context.settings.get_language() {
        Language::German => ",".into(),
        Language::English => ".".into(),
    }
}

pub fn get_argument_separator(context: &Context) -> String {
    match context.settings.get_language() {
        Language::German => ";".into(),
        Language::English => ",".into(),
    }
}
