use leptos::prelude::{Callable, Callback};
use web_sys::KeyboardEvent;

use crate::{
    control::focus_ring::{FocusRing, FocusRingOrientation},
    widget_id::use_widget_id,
};

use super::MenuDefinition;

#[derive(Debug, Clone)]
pub struct MenuContext {
    widget_id: usize,
    is_open: bool,
    focus_ring: FocusRing,
    on_cancel: Callback<()>,
    on_submit: Callback<()>,
}

impl MenuContext {
    pub fn new(
        definition: &MenuDefinition,
        on_cancel: Callback<()>,
        on_submit: Callback<()>,
    ) -> Self {
        Self {
            widget_id: use_widget_id(),
            is_open: false,
            on_cancel,
            on_submit,
            focus_ring: FocusRing::new(
                0,
                definition.items().len(),
                FocusRingOrientation::Vertical,
            ),
        }
    }

    pub fn widget_id(&self) -> usize {
        self.widget_id
    }

    pub fn get_position(&self) -> usize {
        self.focus_ring.get_position()
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.focus_ring.reset_position();
        self.is_open = false;
    }

    pub fn toggle(&mut self) {
        if self.is_open {
            self.close();
        } else {
            self.open();
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn cancel(&mut self) {
        self.close();
        self.on_cancel.run(())
    }

    pub fn submit(&mut self) {
        self.close();
        self.on_submit.run(());
    }

    pub fn on_keydown(&mut self, event: KeyboardEvent, action: Callback<()>) {
        let key = event.key();
        match key.as_str() {
            " " | "Enter" => {
                self.submit();
                action.run(());
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
