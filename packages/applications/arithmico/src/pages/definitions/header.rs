use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    common::breadcrumbs::{Breadcrumbs, BreadcrumbsItem},
    container::page_header::PageHeader,
};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <PageHeader>
            <Breadcrumbs>
                <BreadcrumbsItem href="/">
                    <FormattedMessage id="calculator.title" />
                </BreadcrumbsItem>
                <BreadcrumbsItem href="/definitions" current=true>
                    <FormattedMessage id="definitions.title" />
                </BreadcrumbsItem>
            </Breadcrumbs>
        </PageHeader>
    }
}
