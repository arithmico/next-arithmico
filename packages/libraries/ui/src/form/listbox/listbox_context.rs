use leptos::prelude::{Callable, Callback};
use web_sys::KeyboardEvent;

use crate::{
    control::focus_ring::{FocusRing, FocusRingOrientation},
    widget_id::use_widget_id,
};

use super::ListboxDefinition;

#[derive(Debug, Clone)]
pub struct ListboxContext<V: PartialEq + Send + Sync + Clone + 'static> {
    widget_id: usize,
    is_open: bool,
    focus_ring: FocusRing,
    on_change: Callback<V>,
    on_cancel: Callback<()>,
}

impl<V: PartialEq + Send + Sync + Clone + 'static> ListboxContext<V> {
    pub fn new(
        definition: &ListboxDefinition<V>,
        value: &V,
        on_change: Callback<V>,
        on_cancel: Callback<()>,
    ) -> Self {
        let initial_position = definition.position_of(value).expect("position");
        Self {
            widget_id: use_widget_id(),
            is_open: false,
            focus_ring: FocusRing::new(
                initial_position,
                definition.len(),
                FocusRingOrientation::Vertical,
            ),
            on_change,
            on_cancel,
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn set_position(&mut self, position: usize) {
        self.focus_ring.set_position(position);
    }

    pub fn get_position(&self) -> usize {
        self.focus_ring.get_position()
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.focus_ring.reset_position();
    }

    pub fn toggle(&mut self) {
        if self.is_open {
            self.close();
        } else {
            self.is_open = true;
        }
    }

    pub fn widget_id(&self) -> usize {
        self.widget_id
    }

    pub fn select(&mut self, value: &V) {
        self.close();
        self.on_change.run(value.clone());
    }

    pub fn cancel(&mut self) {
        self.close();
        self.on_cancel.run(());
    }

    pub fn on_keydown(&mut self, event: KeyboardEvent, value: &V) {
        let key = event.key();
        match key.as_str() {
            " " | "Enter" => {
                self.select(value);
                event.prevent_default();
            }
            "Tab" => {
                event.prevent_default();
            }
            "Escape" => {
                self.cancel();
            }
            _ => {
                self.focus_ring.handle_keydown(&key);
            }
        }
    }
}
