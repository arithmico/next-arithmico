use app::App;
use leptos::*;

mod app;
mod pages;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
