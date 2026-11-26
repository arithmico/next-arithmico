use crate::{
    components::*,
    pages::about::{contact::Contact, version::VersionDetails},
};
use leptos::prelude::*;
use team::TeamMembers;
use translate::FormattedMessage;
use ui::{common::page_title::PageTitle, container::page_header::PageHeader};

mod contact;
mod team;
mod version;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <PageWithSidebar class="about-page">
            <PageHeader>
                <PageTitle>
                    <FormattedMessage id="about.title" />
                </PageTitle>
            </PageHeader>

            <VersionDetails />

            <Contact />

            <TeamMembers />
        </PageWithSidebar>
    }
}
