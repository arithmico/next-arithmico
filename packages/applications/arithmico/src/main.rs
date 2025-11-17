use app::App;
use leptos::prelude::*;

mod app;
mod app_shell;
mod components;
mod pages;
mod state;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
