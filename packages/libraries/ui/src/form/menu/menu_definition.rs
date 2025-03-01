use leptos::prelude::*;

#[derive(Clone)]
pub struct MenuItemDefinition {
    view: ViewFn,
    action: Callback<()>,
}

impl MenuItemDefinition {
    pub fn new(view: ViewFn, action: Callback<()>) -> Self {
        Self { view, action }
    }

    pub fn view(&self) -> AnyView {
        self.view.run()
    }

    pub fn action(&self) {
        self.action.run(());
    }

    pub fn action_callback(&self) -> Callback<()> {
        self.action
    }
}

#[derive(Clone)]
pub struct MenuDefinition {
    button: ViewFn,
    items: Vec<MenuItemDefinition>,
}

impl MenuDefinition {
    pub fn new() -> Self {
        Self {
            button: (|| ().into_view()).into(),
            items: Vec::new(),
        }
    }

    pub fn button(mut self, view: impl Into<ViewFn>) -> Self {
        self.button = view.into();
        self
    }

    pub fn item(
        mut self,
        view: impl Into<ViewFn>,
        action: impl Into<Callback<()>>,
    ) -> Self {
        self.items
            .push(MenuItemDefinition::new(view.into(), action.into()));
        self
    }

    pub fn button_view(&self) -> ViewFn {
        self.button.clone()
    }

    pub fn items(&self) -> &[MenuItemDefinition] {
        &self.items
    }
}
