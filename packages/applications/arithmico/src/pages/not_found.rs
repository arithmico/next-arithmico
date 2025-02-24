use crate::components::*;
use leptos::prelude::*;
use translate::FormattedMessage;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <PageWithSidebar>
            <PageTitle>
                <FormattedMessage id="not-found.title" />
            </PageTitle>
            <p>
                <FormattedMessage id="not-found.description" />
            </p>
        </PageWithSidebar>
    }
}
