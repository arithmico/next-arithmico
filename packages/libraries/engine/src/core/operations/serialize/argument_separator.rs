use translate::Language;

use crate::core::Context;

pub fn get_argument_separator(context: &Context) -> String {
    match context.language {
        Language::German => String::from("; "),
        Language::English => String::from(", "),
    }
}
