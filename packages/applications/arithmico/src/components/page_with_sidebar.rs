use crate::components::*;
use leptos::prelude::*;
use translate::FormattedMessage;
use ui::container::page::Page;

#[component]
pub fn PageWithSidebar(children: Children) -> impl IntoView {
    view! {
        <Page>
            <Sidebar>
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
