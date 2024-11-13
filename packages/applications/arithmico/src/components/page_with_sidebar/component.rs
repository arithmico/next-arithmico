use crate::components::*;
use leptos::*;
use translate::FormattedMessage;

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
            <main class="px-4">{children()}</main>
        </Page>
    }
}
