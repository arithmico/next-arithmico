use leptos::prelude::*;

#[derive(Clone)]
pub struct ListboxOption<V: Send + Sync + Clone> {
    pub value: V,
    pub view: ViewFn,
}

impl<V: Send + Sync + Clone> ListboxOption<V> {
    fn new(value: V, view: ViewFn) -> Self {
        Self { value, view }
    }

    pub fn view(mut self, view: impl Into<ViewFn>) -> Self {
        self.view = view.into();
        self
    }
}

pub struct ListboxOptionBuilder;

impl ListboxOptionBuilder {
    fn new() -> Self {
        Self
    }

    pub fn value<V: Send + Sync + Clone>(self, value: V) -> ListboxOption<V> {
        ListboxOption::new(value, (|| ().into_view()).into())
    }
}

#[derive(Clone)]
pub struct ListboxDefinition<V: Send + Sync + Clone> {
    pub button: ViewFn,
    pub options: Vec<ListboxOption<V>>,
}

impl<V: Send + Sync + Clone> ListboxDefinition<V> {
    pub fn new() -> Self {
        Self {
            button: (|| ().into_view()).into(),
            options: Vec::new(),
        }
    }

    pub fn button(mut self, view: impl Into<ViewFn>) -> Self {
        self.button = view.into();
        self
    }

    pub fn option(
        mut self,
        option: impl Fn(ListboxOptionBuilder) -> ListboxOption<V>,
    ) -> Self {
        self.options.push(option(ListboxOptionBuilder::new()));
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
