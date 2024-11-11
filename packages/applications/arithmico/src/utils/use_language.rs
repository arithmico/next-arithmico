use common::Language;
use leptos::{Signal, SignalGet};

use super::expect_app_state;

pub fn use_language() -> Signal<Language> {
    let context = expect_app_state();
    Signal::derive(move || context.get().settings.language)
}
