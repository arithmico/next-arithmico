use leptos::prelude::*;

#[derive(Clone)]
pub struct ListboxOption<V: Send + Sync + Clone> {
    pub value: V,
    pub view: ViewFn,
    pub class: Signal<String>,
}

impl<V: Send + Sync + Clone> ListboxOption<V> {
    fn new(value: V, view: ViewFn) -> Self {
        Self {
            value,
            view,
            class: Signal::derive(|| String::new()),
        }
    }

    pub fn view(mut self, view: impl Into<ViewFn>) -> Self {
        self.view = view.into();
        self
    }

    pub fn class<S: ToString, F: Fn() -> S + Send + Sync + 'static>(
        mut self,
        class: F,
    ) -> Self {
        self.class = Signal::derive(move || class().to_string());
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
pub struct ListboxButton {
    pub view: ViewFn,
    pub class: Signal<String>,
}

impl ListboxButton {
    fn new() -> Self {
        Self {
            view: (|| ().into_view()).into(),
            class: Signal::derive(|| String::new()),
        }
    }

    pub fn view(mut self, view: impl Into<ViewFn>) -> Self {
        self.view = view.into();
        self
    }

    pub fn class<S: ToString, F: Fn() -> S + Send + Sync + 'static>(
        mut self,
        class: F,
    ) -> Self {
        self.class = Signal::derive(move || class().to_string());
        self
    }
}

#[derive(Clone)]
pub struct ListboxOptions<V: Send + Sync + Clone> {
    pub options: Vec<ListboxOption<V>>,
    pub class: Signal<String>,
}

impl<V: Send + Sync + Clone> ListboxOptions<V> {
    fn new() -> Self {
        Self {
            options: Vec::new(),
            class: Signal::derive(|| String::new()),
        }
    }

    pub fn option(
        mut self,
        option: impl Fn(ListboxOptionBuilder) -> ListboxOption<V>,
    ) -> Self {
        self.options.push(option(ListboxOptionBuilder::new()));
        self
    }

    pub fn class<S: ToString, F: Fn() -> S + Send + Sync + 'static>(
        mut self,
        class: F,
    ) -> Self {
        self.class = Signal::derive(move || class().to_string());
        self
    }
}

#[derive(Clone)]
pub struct ListboxDefinition<V: Send + Sync + Clone> {
    pub button: ListboxButton,
    pub options: ListboxOptions<V>,
}

impl<V: Send + Sync + Clone> ListboxDefinition<V> {
    pub fn new() -> Self {
        Self {
            button: ListboxButton::new(),
            options: ListboxOptions::new(),
        }
    }

    pub fn button(
        mut self,
        button: impl Fn(ListboxButton) -> ListboxButton,
    ) -> Self {
        self.button = button(ListboxButton::new());
        self
    }

    pub fn options(
        mut self,
        options: impl Fn(ListboxOptions<V>) -> ListboxOptions<V>,
    ) -> Self {
        self.options = options(self.options);
        self
    }
}
