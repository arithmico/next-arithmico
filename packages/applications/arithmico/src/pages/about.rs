use crate::components::*;
use leptos::prelude::*;
use team::TeamMembers;
use translate::FormattedMessage;
use ui::common::page_title::PageTitle;

mod team;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <PageWithSidebar>
            <PageTitle>
                <FormattedMessage id="about.title" />
            </PageTitle>

            <TeamMembers />
        </PageWithSidebar>
    }
}
