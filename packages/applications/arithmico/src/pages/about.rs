use crate::{components::*, pages::about::version::VersionDetails};
use leptos::prelude::*;
use team::TeamMembers;
use translate::FormattedMessage;
use ui::common::page_title::PageTitle;

mod team;
mod version;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <PageWithSidebar class="about-page">
            <PageTitle>
                <FormattedMessage id="about.title" />
            </PageTitle>

            <VersionDetails />

            <TeamMembers />
        </PageWithSidebar>
    }
}
