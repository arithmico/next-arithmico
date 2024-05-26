use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageTitleProps {
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn PageTitle(props: &PageTitleProps) -> Html {
    html!(
        <h1 class={classes!("py-4", "pr-10", "text-2xl", props.class.clone())}>
            {props.children.clone()}
        </h1>
    )
}
