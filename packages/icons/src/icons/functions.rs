use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FunctionsIconProps {
    #[prop_or_default]
    pub class: Option<Classes>,
}

#[function_component]
pub fn FunctionsIcon(props: &FunctionsIconProps) -> Html {
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
            <path d="M280-200v-32.31L523.85-480 280-727.69V-760h400v50H364.08l225 230-225 230.77H680V-200H280Z"/>
        </svg>

    }
}
