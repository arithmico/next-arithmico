use crate::components::*;
use leptos::*;

#[component]
pub fn PageWithSidebar(children: Children) -> impl IntoView {
    view! {
        <Page>
            <Sidebar>
                <Navigation>
                    <NavigationItem to="/".to_string()>Calculator</NavigationItem>
                    <NavigationItem to="/settings".to_string()>Settings</NavigationItem>
                    <NavigationItem to="/reference".to_string()>Reference</NavigationItem>
                    <NavigationItem to="/about".to_string()>About</NavigationItem>
                </Navigation>
            </Sidebar>
            <main>{children()}</main>
        </Page>
    }
}
