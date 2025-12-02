use wasm_bindgen::{closure::Closure, JsCast};
use web_state::WebStateAction;
use web_sys::MediaQueryListEvent;

use crate::state::{theme::Theme, State};

pub fn evaluate_theme<'a>() -> &'a str {
    let is_dark = web_sys::window()
        .and_then(|window| {
            window
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
        })
        .map(|media_query| media_query.matches())
        .unwrap_or(false);

    if is_dark {
        "theme-dark"
    } else {
        "theme-light"
    }
}

pub struct SetThemeAction {
    value: Theme,
}

impl SetThemeAction {
    pub fn new(value: impl Into<Theme>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl WebStateAction<State> for SetThemeAction {
    fn apply(&self, state: &mut State) {
        state.settings.theme = match self.value {
            Theme::System => {
                let window = web_sys::window().expect("missing window");
                let media_query = window
                    .match_media("(prefers-color-scheme: dark)")
                    .unwrap()
                    .expect("match_media should exist");
                let element =
                    window.document().unwrap().document_element().unwrap();
                element.set_class_name(evaluate_theme());

                let window_cloned = window.clone();
                let closure = Closure::wrap(Box::new(
                    move |_event: MediaQueryListEvent| {
                        let element = window_cloned
                            .document()
                            .unwrap()
                            .document_element()
                            .expect("html element should exist");
                        element.set_class_name(evaluate_theme());
                    },
                )
                    as Box<dyn FnMut(_)>);

                media_query
                    .add_event_listener_with_callback(
                        "change",
                        closure.as_ref().unchecked_ref(),
                    )
                    .expect("could register listener");
                closure.forget();

                Theme::System
            }
            _ => self.value.clone(),
        };
        state.settings.save();
    }
}
