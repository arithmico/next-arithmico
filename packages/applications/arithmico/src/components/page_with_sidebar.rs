use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    common::{navigation::Navigation, navigation_item::NavigationItem},
    container::{page::Page, sidebar::Sidebar},
};

#[component]
pub fn PageWithSidebar(children: Children) -> impl IntoView {
    view! {
        <Page>
            <Sidebar title="Arithmico">
                <Navigation>
                    <NavigationItem to="/".to_string()>
                        <FormattedMessage id="navigation.calculator" />
                    </NavigationItem>
                    <NavigationItem to="/settings".to_string()>
                        <FormattedMessage id="navigation.settings" />
                    </NavigationItem>
                    <NavigationItem to="/reference".to_string()>
                        <FormattedMessage id="navigation.reference" />
                    </NavigationItem>
                    <NavigationItem to="/about".to_string()>
                        <FormattedMessage id="navigation.about" />
                    </NavigationItem>
                </Navigation>
            </Sidebar>
            <main class="page-with-sidebar-container">{children()}</main>
        </Page>
    }
}
