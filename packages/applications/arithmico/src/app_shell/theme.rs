use leptos::*;

use crate::utils::use_app_state;

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let state = use_app_state();

    view! { <div class=move || state.get().settings.theme.get_class().to_string()>{children()}</div> }
}
