use yew::{classes, function_component, html, Html, Properties};
use yew_router::hooks::use_route;

use crate::{components::Link, router::Route};

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub children: Html,
}

#[function_component]
pub fn Navbar(props: &NavbarProps) -> Html {
    html!(
        <nav class={classes!("w-full", "flex", "justify-center", "items-center")}>
            <div class={classes!("w-3/5", "flex", "items-center", "h-full")}>
                <h1 class={classes!("pr-4", "py-4", "text-3xl")}>{"Arithmico"}</h1>
                <ul class={classes!("flex", "ml-auto", "h-full")}>
                    {props.children.clone()}
                </ul>
            </div>
        </nav>
    )
}

#[derive(PartialEq, Properties)]
pub struct NavbarLinkProps {
    pub to: Route,
    pub children: Html,
}

#[function_component]
pub fn NavbarLink(props: &NavbarLinkProps) -> Html {
    let route = use_route::<Route>();
    let active_classes = {
        let link_route = props.to.clone();
        route.and_then(|route| {
            if route == link_route {
                Some("bg-neutral-200")
            } else {
                None
            }
        })
    };

    html! {
        <li class={classes!("flex", "h-full", "ml-2")}>
            <Link
                class={classes!(
                    "px-8",
                    "hover:bg-neutral-200",
                    "flex", "items-center",
                    "h-full",
                    "rounded-b-md",
                    active_classes
                )}
                to={props.to.clone()}
            >
                {props.children.clone()}
            </Link>
        </li>
    }
}
