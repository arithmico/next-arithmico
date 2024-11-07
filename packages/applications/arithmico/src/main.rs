use app::App;
use leptos::*;

mod app;
mod components;
mod pages;
mod router;
mod state;
mod utils;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
