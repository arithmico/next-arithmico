use leptos::{prelude::expect_context, reactive::wrappers::read::Signal};
use translate_core::Language;

use crate::TranslateContext;

pub fn use_language() -> Signal<Language> {
    let context = expect_context::<TranslateContext>();
    Signal::derive(move || context.get_current_language())
}
