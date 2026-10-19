use leptos::prelude::*;
use leptos_md::Markdown;
use translate::{FormattedMessage, use_translate};
use ui::{
    common::breadcrumbs::{Breadcrumbs, BreadcrumbsItem},
    container::page_header::PageHeader,
};

use crate::components::PageWithSidebar;

#[component]
pub fn LicensePage() -> impl IntoView {
    let translate = use_translate();

    view! {
        <PageWithSidebar>
            <PageHeader>
                <Breadcrumbs attr:aria-label=move || {
                    translate("breadcrumbs.label", None).unwrap_or_default()
                }>
                    <BreadcrumbsItem href="/about">
                        <FormattedMessage id="about.title" />
                    </BreadcrumbsItem>
                    <BreadcrumbsItem href="/about/license" current=true>
                        <FormattedMessage id="license.title" />
                    </BreadcrumbsItem>
                </Breadcrumbs>
            </PageHeader>
            <Markdown content=include_str!("../../../../../LICENSE.md") />
        </PageWithSidebar>
    }
}
