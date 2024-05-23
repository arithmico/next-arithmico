use yew::prelude::*;
use yew_router::components::Link as YewLink;

use crate::router::Route;

#[derive(PartialEq, Properties)]
pub struct LinkProps {
    pub to: Route,
    pub class: Classes,
    pub children: Html,
}

#[function_component]
pub fn Link(props: &LinkProps) -> Html {
    html! {
        <YewLink<Route>
            classes={props.class.clone()}
            to={props.to.clone()}
        >{props.children.clone()}</YewLink<Route>>
    }
}
