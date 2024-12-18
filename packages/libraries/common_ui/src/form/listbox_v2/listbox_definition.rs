use leptos::prelude::*;

pub struct ListboxOption<T: Send + Sync> {
    value: T,
    class: Option<String>,
    children: Children,
}

impl<T: Send + Sync> ListboxOption<T> {
    pub fn new(value: T, children: Children) -> Self {
        Self {
            value,
            class: None,
            children,
        }
    }

    pub fn class(mut self, class: impl ToString) -> Self {
        self.class = Some(class.to_string());
        self
    }
}

pub struct ListboxOptions<T: Send + Sync> {
    options: Vec<ListboxOption<T>>,
    class: Option<String>,
}

impl<T: Send + Sync> ListboxOptions<T> {
    pub fn new() -> Self {
        Self {
            options: vec![],
            class: None,
        }
    }

    pub fn option(mut self, option: ListboxOption<T>) -> Self {
        self.options.push(option);
        self
    }

    pub fn class(mut self, class: impl ToString) -> Self {
        self.class = Some(class.to_string());
        self
    }
}

pub struct ListboxButton {
    children: Children,
    class: Option<String>,
}

impl ListboxButton {
    pub fn new(children: Children) -> Self {
        Self {
            children,
            class: None,
        }
    }

    pub fn class(mut self, class: impl ToString) -> Self {
        self.class = Some(class.to_string());
        self
    }
}

pub struct ListboxDefinition<T: Send + Sync> {
    options: ListboxOptions<T>,
    button: ListboxButton,
}
