use leptos::prelude::*;

#[derive(Clone)]
pub struct ListboxOptionDefinition<V: Send + Sync + Clone> {
    pub value: V,
    pub view: ViewFn,
}

impl<V: Send + Sync + Clone> ListboxOptionDefinition<V> {
    fn new(value: V, view: ViewFn) -> Self {
        Self { value, view }
    }

    pub fn view(mut self, view: impl Into<ViewFn>) -> Self {
        self.view = view.into();
        self
    }
}

#[derive(Clone)]
pub struct ListboxDefinition<V: Send + Sync + Clone> {
    pub label: ViewFn,
    pub button: ViewFn,
    pub options: Vec<ListboxOptionDefinition<V>>,
}

impl<V: Send + Sync + Clone> ListboxDefinition<V> {
    pub fn new() -> Self {
        Self {
            label: (|| ().into_view()).into(),
            button: (|| ().into_view()).into(),
            options: Vec::new(),
        }
    }

    pub fn label(mut self, view: impl Into<ViewFn>) -> Self {
        self.label = view.into();
        self
    }

    pub fn button(mut self, view: impl Into<ViewFn>) -> Self {
        self.button = view.into();
        self
    }

    pub fn option(mut self, value: V, view: impl Into<ViewFn>) -> Self {
        self.options
            .push(ListboxOptionDefinition::new(value, view.into()));
        self
    }

    pub fn len(&self) -> usize {
        self.options.len()
    }
}

impl<V: Send + Sync + Clone + PartialEq> ListboxDefinition<V> {
    pub fn position_of(&self, value: &V) -> Option<usize> {
        self.options.iter().position(|x| &x.value == value)
    }
}
