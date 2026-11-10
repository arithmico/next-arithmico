use aws_config::SdkConfig;
use jsonwebtoken::EncodingKey;
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;
use tracing::{error, instrument};

#[derive(Debug, Deserialize)]
struct GithubAppSecretPayload {
    webhook_secret: SecretBox<String>,
    client_id: SecretBox<String>,
    private_key: SecretBox<String>,
}

#[derive(Debug)]
pub struct GithubAppSecret {
    webhook_secret: SecretBox<String>,
    client_id: SecretBox<String>,
    encoding_key: EncodingKey,
}

impl GithubAppSecret {
    #[instrument(skip(sdk_config))]
    pub async fn load(
        secret_arn: &str,
        sdk_config: &SdkConfig,
    ) -> Result<Self, crate::Error> {
        let sm_client = aws_sdk_secretsmanager::Client::new(sdk_config);

        let github_app_secret = sm_client
            .get_secret_value()
            .secret_id(secret_arn)
            .send()
            .await
            .map_err(|err| {
                error!(
                    message = "Failed to get github app secret",
                    error = ?err
                );
                crate::Error::MissingGithubAppSecret
            })?
            .secret_string
            .ok_or_else(|| crate::Error::MissingGithubAppSecret)?;

        let payload: GithubAppSecretPayload =
            serde_json::from_str(&github_app_secret)?;

        let encoding_key = EncodingKey::from_rsa_pem(
            payload.private_key.expose_secret().as_bytes(),
        )
        .inspect_err(|err| {
            error!(
                message = "Failed to create jwt encoding key for github app",
                error = ?err
            );
        })?;

        Ok(Self {
            webhook_secret: payload.webhook_secret,
            client_id: payload.client_id,
            encoding_key,
        })
    }

    pub fn client_id(&self) -> &str {
        self.client_id.expose_secret().as_str()
    }

    pub fn webhook_secret(&self) -> &str {
        self.webhook_secret.expose_secret().as_str()
    }

    pub fn encoding_key(&self) -> &EncodingKey {
        &self.encoding_key
    }
}
