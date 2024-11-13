use crate::components::*;
use leptos::*;
use translate::FormattedMessage;

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
