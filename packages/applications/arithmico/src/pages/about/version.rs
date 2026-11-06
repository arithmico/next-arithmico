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
                <dt>
                    <FormattedMessage id="about.version.version" />
                </dt>
                <dd>{option_env!("APP_VERSION").unwrap_or("DEV")}</dd>
                <dt>
                    <FormattedMessage id="about.version.source_date_epoch" />
                </dt>
                <dd>
                    {get_source_date_epoch().format("%d.%m.%Y").to_string()}
                </dd>
                <dt>
                    <FormattedMessage id="about.version.commit_hash" />
                </dt>
                <dd>{env!("COMMIT_HASH")}</dd>
                <dt>
                    <FormattedMessage id="about.version.sbom" />
                </dt>
                <dd>
                    <a href="/sbom.json" download>
                        SBOM
                    </a>
                </dd>
                <dt>
                    <FormattedMessage id="about.version.license" />
                </dt>
                <dd>
                    <A href="/about/license">AGPL v3</A>
                </dd>
                <dt>
                    <FormattedMessage id="about.version.repository" />
                </dt>
                <dd>
                    <a href="https://github.com/arithmico/next-arithmico">
                        GitHub
                    </a>
                </dd>
                <dt>
                    <FormattedMessage id="about.version.font_license" />
                </dt>
                <dd>
                    <A href="/about/font_license">SIL Open Font License</A>
                </dd>
            </dl>
        </section>
    }
}
