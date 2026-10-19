use leptos::prelude::*;

use crate::icon::chevron_right_icon::ChevronRightIcon;

#[component]
pub fn Breadcrumbs(children: Children) -> impl IntoView {
    view! {
        <nav class="breadcrumbs">
            <ol>{children()}</ol>
        </nav>
    }
}

#[component]
pub fn BreadcrumbsItem(
    children: Children,
    #[prop(into)] href: String,
    #[prop(default = false)] current: bool,
) -> impl IntoView {
    view! {
        <li class=format!(
            "breadcrumbs-item {}",
            if current { "current" } else { "" },
        )>
            <ChevronRightIcon class="breadcrumb-separator" />
            <a href=href aria_current=current>
                {children()}
            </a>
        </li>
    }
}
