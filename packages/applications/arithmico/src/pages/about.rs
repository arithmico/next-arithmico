use crate::components::*;
use leptos::prelude::*;
use translate::FormattedMessage;
use ui::common::page_title::PageTitle;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <PageWithSidebar>
            <PageTitle>
                <FormattedMessage id="about.title" />
            </PageTitle>
        </PageWithSidebar>
    }
}
