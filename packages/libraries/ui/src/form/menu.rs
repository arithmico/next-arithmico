use leptos::prelude::*;

pub struct MenuDefinition {
    button: ViewFn,
    items: Vec<ViewFn>,
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

    pub fn item(mut self, view: impl Into<ViewFn>) -> Self {
        self.items.push(view.into());
        self
    }
}

#[component]
pub fn Menu(definition: MenuDefinition) -> impl IntoView {
    let is_open = RwSignal::new(false);

    view! {
        <div class="menu-container">
            <button
                class="menu-button"
                on:click=move |_| {
                    is_open.set(!is_open.get());
                }
            >
                {definition.button.run()}
            </button>
            <div class="menu-items-container">
                <Show when=move || is_open.get()>
                    <ul class="menu-items">
                        {definition
                            .items
                            .iter()
                            .enumerate()
                            .map(|(_pos, item)| {
                                view! { <li class="menu-item">{item.run()}</li> }
                            })
                            .collect_view()}
                    </ul>
                </Show>
            </div>
        </div>
    }
}
