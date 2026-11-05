use anyhow::anyhow;
use serde::Deserialize;
use std::time::Duration;

use crate::{config::Report, show::normalize};

#[derive(Debug, Deserialize)]
struct IdTokenResponse {
    value: String,
}

pub fn report_features(args: Report) -> anyhow::Result<()> {
    let response = ureq::get(args.actions_id_token_request_url)
        .header(
            "Authorization",
            format!("bearer {}", args.actions_id_token_request_token),
        )
        .query("audience", "https://api.arithmico.com/github/features")
        .config()
        .timeout_global(Some(Duration::from_secs(30)))
        .build()
        .call()?
        .body_mut()
        .read_to_string()?;

    let response: IdTokenResponse = serde_json::from_str(&response)?;
    let features = normalize()?;
    let body = serde_json::to_string(&features)?;

    let response = ureq::put(args.config_service_report_url)
        .header("Authorization", format!("bearer {}", response.value))
        .content_type("application/json")
        .config()
        .timeout_global(Some(Duration::from_secs(30)))
        .build()
        .send(body)?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to report features to config service"));
    }

    Ok(())
}
