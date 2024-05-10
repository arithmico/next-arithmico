use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct PageProps {
    pub children: Html,
}

#[function_component]
pub fn Page(props: &PageProps) -> Html {
    html!(
        <div class={classes!("absolute", "inset-0", "grid", "grid-rows-[auto_1fr]")}>
            {props.children.clone()}
        </div>
    )
}
