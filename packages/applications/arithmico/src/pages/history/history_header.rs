use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    common::breadcrumbs::{Breadcrumbs, BreadcrumbsItem},
    container::page_header::PageHeader,
    icon::chevron_right_icon::ChevronRightIcon,
};

#[component]
pub fn HistoryHeader() -> impl IntoView {
    view! {
        <PageHeader>
            <Breadcrumbs>
                <BreadcrumbsItem href="/">
                    <FormattedMessage id="calculator.title" />
                </BreadcrumbsItem>
                <ChevronRightIcon />
                <BreadcrumbsItem href="/history" current=true>
                    <FormattedMessage id="history.title" />
                </BreadcrumbsItem>
            </Breadcrumbs>
        </PageHeader>
    }
}
