use yew::prelude::*;

use crate::components::Link;
use icons::chevron_right::ChevronRightIcon;

#[derive(Properties, PartialEq)]
pub struct BreadcrumbsProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<BreadcrumbsItem>,
}

#[function_component()]
pub fn Breadcrumbs(props: &BreadcrumbsProps) -> Html {
    let length = props.children.len();
    let children = props
        .children
        .iter()
        .enumerate()
        .map(move |(pos, child)| {
            let last = pos == length - 1;
            html!(
                <BreadcrumbsItem to={child.props.to.clone()} last={last}>
                    {child.props.children.clone()}
                </BreadcrumbsItem>
            )
        })
        .collect::<Html>();

    html! {
        <nav class={classes!("py-4", "pr-10", "text-2xl")}>
            <ol class={classes!("flex")}>
                {children}
            </ol>
        </nav>
    }
}

use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct BreadcrumbsItemProps {
    pub children: Children,
    pub to: Route,
    #[prop_or(false)]
    pub last: bool,
}

#[function_component()]
pub fn BreadcrumbsItem(props: &BreadcrumbsItemProps) -> Html {
    html! {
        <li class={classes!("flex", "items-center")}>
            {
                html!(
                    <>
                        <Link to={props.to.clone()} class={classes!(
                            (!props.last).then_some("text-neutral-400")
                        )}>
                            {props.children.clone()}
                        </Link>

                        if !props.last {
                            <ChevronRightIcon class={classes!("fill-neutral-400", "w-8", "h-8")} />
                        }
                    </>
                )
            }
        </li>
    }
}
