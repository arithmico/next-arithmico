use yew::{classes, function_component, html, Children, Html, Properties};
use yew_router::hooks::use_route;

use crate::{components::Link, router::Route};

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub children: Children,
    #[prop_or_default]
    pub actions: Children,
}

#[function_component]
pub fn Navbar(props: &NavbarProps) -> Html {
    html!(
        <div class={classes!(
            "w-full",
            "flex",
            "flex-col",
            "bg-white",
            "border-r",
            "border-neutral-300",
            "px-2"
        )}>
            <h1 class={classes!("py-4", "pr-10", "text-2xl", "font-light")}>{"Arithmico"}</h1>
            <h2 class={classes!(
                "font-bold",
                "my-2",
                "text-sm"
            )}>{"Navigation"}</h2>
            <nav class={classes!("flex", "w-full")}>
                <ul class={classes!("flex", "flex-col", "w-full", "gap-1")}>
                    {props.children.clone()}
                </ul>
            </nav>
            {props.actions.clone()}
        </div>
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
        <li class={classes!("flex", "flex-col")}>
            <Link
                class={classes!(
                    "py-2",
                    "px-8",
                    "hover:bg-neutral-200",
                    "flex",
                    "items-center",
                    "rounded-md",
                    active_classes
                )}
                to={props.to.clone()}
            >
                {props.children.clone()}
            </Link>
        </li>
    }
}
