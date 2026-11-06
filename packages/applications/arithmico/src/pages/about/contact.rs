use leptos::prelude::*;
use translate::FormattedMessage;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section class="contact">
            <h2>
                <FormattedMessage id="about.contact" />
            </h2>
            <p>
                <FormattedMessage id="about.contact.description" />
            </p>
            <a href="mailto:arithmico@blista.de">
                <FormattedMessage id="about.contact.email" />
            </a>
        </section>
    }
}
