use leptos::*;

#[component]
pub fn MenuIcon(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <svg
            aria-hidden
            xmlns="http://www.w3.org/2000/svg"
            height="24px"
            viewBox="0 -960 960 960"
            width="24px"
            fill="#e8eaed"
            class=class
        >
            <path d="M480-528 296-344l-56-56 240-240 240 240-56 56-184-184Z" />
        </svg>
    }
}

#[component]
pub fn ListIcon(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <svg
            aria-hidden
            xmlns="http://www.w3.org/2000/svg"
            height="24px"
            viewBox="0 -960 960 960"
            width="24px"
            fill="#e8eaed"
            class=class
        >
            <path d="M360-240h440v-107H360v107ZM160-613h120v-107H160v107Zm0 187h120v-107H160v107Zm0 186h120v-107H160v107Zm200-186h440v-107H360v107Zm0-187h440v-107H360v107ZM160-160q-33 0-56.5-23.5T80-240v-480q0-33 23.5-56.5T160-800h640q33 0 56.5 23.5T880-720v480q0 33-23.5 56.5T800-160H160Z" />
        </svg>
    }
}

#[component]
pub fn HistoryIcon(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <svg
            aria-hidden
            xmlns="http://www.w3.org/2000/svg"
            height="24px"
            viewBox="0 -960 960 960"
            width="24px"
            fill="#e8eaed"
            class=class
        >
            <path d="M480-120q-138 0-240.5-91.5T122-440h82q14 104 92.5 172T480-200q117 0 198.5-81.5T760-480q0-117-81.5-198.5T480-760q-69 0-129 32t-101 88h110v80H120v-240h80v94q51-64 124.5-99T480-840q75 0 140.5 28.5t114 77q48.5 48.5 77 114T840-480q0 75-28.5 140.5t-77 114q-48.5 48.5-114 77T480-120Zm112-192L440-464v-216h80v184l128 128-56 56Z" />
        </svg>
    }
}
