use leptos::*;
use pages::calculator::CalculatorPage;

pub mod pages;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <CalculatorPage/> })
}
