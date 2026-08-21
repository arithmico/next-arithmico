use leptos::prelude::*;
use leptos_md::Markdown;
use translate::FormattedMessage;
use ui::{
    common::breadcrumbs::{Breadcrumbs, BreadcrumbsItem},
    container::page_header::PageHeader,
    icon::chevron_right_icon::ChevronRightIcon,
};

use crate::components::PageWithSidebar;

#[component]
pub fn FontLicensePage() -> impl IntoView {
    view! {
        <PageWithSidebar>
            <PageHeader>
                <Breadcrumbs>
                    <BreadcrumbsItem href="/about">
                        <FormattedMessage id="about.title" />
                    </BreadcrumbsItem>
                    <ChevronRightIcon />
                    <BreadcrumbsItem href="/about/font_license" current=true>
                        <FormattedMessage id="font_license.title" />
                    </BreadcrumbsItem>
                </Breadcrumbs>
            </PageHeader>
            <Markdown content=include_str!("../../../../../fonts/LICENSE.md") />
        </PageWithSidebar>
    }
}
