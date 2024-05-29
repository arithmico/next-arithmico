use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChevronRightIconProps {
    #[prop_or_default]
    pub class: Option<Classes>,
}

#[function_component()]
pub fn ChevronRightIcon(props: &ChevronRightIconProps) -> Html {
    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            height="24px"
            viewBox="0 -960 960 960"
            width="24px"
            fill="#e8eaed"
            aria-hidden="true"
            class={props.class.clone()}
        >
            <path d="m531.69-480-184-184L376-692.31 588.31-480 376-267.69 347.69-296l184-184Z"/>
        </svg>
    }
}
