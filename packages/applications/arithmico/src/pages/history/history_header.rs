use leptos::prelude::*;
use translate::{FormattedMessage, use_translate};
use ui::{
    common::breadcrumbs::{Breadcrumbs, BreadcrumbsItem},
    container::page_header::PageHeader,
};

#[component]
pub fn HistoryHeader() -> impl IntoView {
    let translate = use_translate();

    view! {
        <PageHeader>
            {move || {
                let aria_label = translate("breadcrumbs.label", None)
                    .unwrap_or_default();
                view! {
                    <Breadcrumbs attr:aria-label=aria_label>
                        <BreadcrumbsItem href="/">
                            <FormattedMessage id="calculator.title" />
                        </BreadcrumbsItem>
                        <BreadcrumbsItem href="/history" current=true>
                            <FormattedMessage id="history.title" />
                        </BreadcrumbsItem>
                    </Breadcrumbs>
                }
            }}
        </PageHeader>
    }
}
