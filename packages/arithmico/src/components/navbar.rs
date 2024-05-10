use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub children: Html,
}

#[function_component]
pub fn Navbar(props: &NavbarProps) -> Html {
    html!(
        <nav class={classes!("w-full", "flex", "justify-center", "items-center", "bg-blue-400")}>
            <h1 class="pr-4">{"Arithmico"}</h1>
            <ul class="flex">
                {props.children.clone()}
            </ul>
        </nav>
    )
}
