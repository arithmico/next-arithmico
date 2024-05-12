use engine::Language;
use yew::{classes, function_component, html, use_context, Html, Properties};

use crate::components::{AppContext, PageWithNavbar};

#[derive(PartialEq, Properties)]
pub struct HelpPageProps {}

#[function_component]
pub fn HelpPage(_props: &HelpPageProps) -> Html {
    let app_context =
        use_context::<AppContext>().expect("app context not found");
    let documentation =
        app_context.host_api.get_documentation(&Language::German);

    html! {
        <PageWithNavbar>
            <h1 class={classes!("text-3xl", "mt-4", "mb-8")}>{"Hilfe"}</h1>
            <ul>
            {
                documentation.iter().map(|item| html!(
                    <li class={classes!("grid", "grid-cols-[30%_70%]", "mb-4")}>
                        <span>{item.synopsis.clone()}</span>
                        <span>{item.description.clone()}</span>
                    </li>
                )).collect::<Html>()
            }
            </ul>
        </PageWithNavbar>
    }
}
