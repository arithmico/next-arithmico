use chrono::{DateTime, Utc};
use leptos::prelude::*;
use leptos_router::components::A;
use translate::FormattedMessage;

fn get_source_date_epoch() -> DateTime<Utc> {
    DateTime::from_timestamp(
        env!("SOURCE_DATE_EPOCH").parse::<i64>().unwrap_or_default(),
        0,
    )
    .unwrap_or_default()
}

#[component]
pub fn VersionDetails() -> impl IntoView {
    view! {
        <section class="version-details">
            <h2>
                <FormattedMessage id="about.version.title" />
            </h2>
            <dl>
                <dd>
                    <FormattedMessage id="about.version.version" />
                </dd>
                <dt>{option_env!("APP_VERSION").unwrap_or("DEV")}</dt>
                <dd>
                    <FormattedMessage id="about.version.source_date_epoch" />
                </dd>
                <dt>
                    {get_source_date_epoch().format("%d.%m.%Y").to_string()}
                </dt>
                <dd>
                    <FormattedMessage id="about.version.commit_hash" />
                </dd>
                <dt>{env!("COMMIT_HASH")}</dt>
                <dd>
                    <FormattedMessage id="about.version.sbom" />
                </dd>
                <dt>
                    <a href="/sbom.json" download>
                        SBOM
                    </a>
                </dt>
                <dd>
                    <FormattedMessage id="about.version.license" />
                </dd>
                <dt>
                    <A href="/about/license">AGPL v3</A>
                </dt>
                <dd>
                    <FormattedMessage id="about.version.repository" />
                </dd>
                <dt>
                    <a href="https://github.com/arithmico/next-arithmico">
                        GitHub
                    </a>
                </dt>
                <dd>
                    <FormattedMessage id="about.version.font_license" />
                </dd>
                <dt>
                    <A href="/about/font_license">SIL Open Font License</A>
                </dt>
            </dl>
        </section>
    }
}
