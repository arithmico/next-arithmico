use leptos::prelude::*;

#[component]
pub fn ArrowLeftIcon(
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
            <path d="m313-440 224 224-57 56-320-320 320-320 57 56-224 224h487v80H313Z" />
        </svg>
    }
}
