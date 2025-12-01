use wasm_bindgen::{closure::Closure, JsCast};
use web_state::WebStateAction;
use web_sys::MediaQueryListEvent;

use crate::state::{theme::Theme, State};

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
                    .expect("match_media failed");
                let element =
                    window.document().unwrap().document_element().unwrap();
                if media_query.matches() {
                    element.set_class_name("theme-dark");
                } else {
                    element.set_class_name("theme-light");
                }

                let window_cloned = window.clone();
                let media_query_cloned = media_query.clone();
                let closure = Closure::wrap(Box::new(
                    move |_event: MediaQueryListEvent| {
                        let element = window_cloned
                            .document()
                            .unwrap()
                            .document_element()
                            .expect("html element should exist");
                        if media_query_cloned.matches() {
                            element.set_class_name("theme-dark");
                        } else {
                            element.set_class_name("theme-light");
                        }
                    },
                )
                    as Box<dyn FnMut(_)>);

                media_query
                    .add_event_listener_with_callback(
                        "change",
                        closure.as_ref().unchecked_ref(),
                    )
                    .expect("could not register listener");

                closure.forget();

                Theme::System
            }
            _ => self.value.clone(),
        };
        state.settings.save();
    }
}
