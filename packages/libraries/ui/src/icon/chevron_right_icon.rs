use leptos::prelude::*;

#[component]
pub fn ChevronRightIcon(
    #[prop(into, default = Signal::derive(|| String::new()))] class: Signal<
        String,
    >,
) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            height="24px"
            viewBox="0 -960 960 960"
            width="24px"
            fill="#e8eaed"
            aria-hidden
            class=move || format!("icon {}", class.get())
        >
            <path d="M504-480 320-664l56-56 240 240-240 240-56-56 184-184Z" />
        </svg>
    }
}
