use leptos::prelude::*;
use leptos_md::Markdown;
use translate::FormattedMessage;
use ui::{
    common::breadcrumbs::{Breadcrumbs, BreadcrumbsItem},
    container::page_header::PageHeader,
};

use crate::components::PageWithSidebar;

#[component]
pub fn LicensePage() -> impl IntoView {
    view! {
        <PageWithSidebar>
            <PageHeader>
                <Breadcrumbs>
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
