use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageProps {
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn Page(props: &PageProps) -> Html {
    html!(
        <div class={classes!(
            "max-h-full",
            "absolute",
            "inset-0",
            "bg-neutral-100",
            props.class.clone()
        )}>
            {props.children.clone()}
        </div>
    )
}
