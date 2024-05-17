use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ExpandIconOptions {
    #[prop_or_default]
    pub class: Option<Classes>,
}

#[function_component]
pub fn ExpandIcon(props: &ExpandIconOptions) -> Html {
    html! {
        <svg
            aria-hidden="true"
            class={props.class.clone()}
            xmlns="http://www.w3.org/2000/svg"
            height="48"
            width="48"
            viewBox="0 0 48 48"
            >
            <path d="m24 30.75-12-12 2.15-2.15L24 26.5l9.85-9.85L36 18.8Z" />
        </svg>
    }
}
