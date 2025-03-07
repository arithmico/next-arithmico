use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{common::page_title::PageTitle, container::page_header::PageHeader};

#[component]
pub fn HistoryHeader() -> impl IntoView {
    view! {
        <PageHeader>
            <PageTitle>
                <FormattedMessage id="history.title" />
            </PageTitle>
        </PageHeader>
    }
}
