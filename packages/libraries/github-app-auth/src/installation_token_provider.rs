use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{
    Algorithm, Header, dangerous::insecure_decode_claims, encode,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{error, instrument};

use crate::github_app_secret::GithubAppSecret;

const USER_AGENT: &str = "arithmico-github-app";

#[derive(Debug)]
struct CachedToken {
    expires_at: Instant,
    token: String,
}

fn timestamp_to_instant(timestamp: u64) -> Result<Instant, crate::Error> {
    let now = Instant::now();
    let now_delta = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let delta = Duration::from_secs(timestamp).saturating_sub(now_delta);
    Ok(now + delta)
}

impl CachedToken {
    pub fn new(token: &str, expires_at: Instant) -> Self {
        Self {
            token: token.to_string(),
            expires_at,
        }
    }

    pub fn try_from_jwt(token: &str) -> Result<Self, crate::Error> {
        #[derive(Debug, Deserialize)]
        struct CachedTokenClaims {
            exp: u64,
        }

        let claims: CachedTokenClaims = insecure_decode_claims(token)?;
        let expires_at = timestamp_to_instant(claims.exp)?;
        Ok(CachedToken {
            expires_at,
            token: token.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct GithubInstallationTokenProvider {
    access_token: Arc<RwLock<CachedToken>>,
    /// Map<installation_id, CachedToken>
    installation_access_tokens: Arc<RwLock<HashMap<u64, CachedToken>>>,
    github_app_secret: Arc<GithubAppSecret>,
    http_client: reqwest::Client,
}

async fn generate_access_token(
    github_app_secret: &GithubAppSecret,
) -> Result<CachedToken, crate::Error> {
    #[derive(Debug, Serialize)]
    struct AccessTokenClaims {
        iss: String,
        iat: u64,
        exp: u64,
    }

    let iat = SystemTime::now() - Duration::from_secs(60);
    let exp = iat + Duration::from_mins(9);

    let claims = AccessTokenClaims {
        iss: github_app_secret.client_id().to_string(),
        iat: iat.duration_since(UNIX_EPOCH)?.as_secs(),
        exp: exp.duration_since(UNIX_EPOCH)?.as_secs(),
    };

    let header = Header::new(Algorithm::RS256);

    let token = encode(&header, &claims, github_app_secret.encoding_key())?;

    CachedToken::try_from_jwt(&token)
}

impl GithubInstallationTokenProvider {
    pub async fn connect(
        github_app_secret: Arc<GithubAppSecret>,
    ) -> Result<Self, crate::Error> {
        let access_token =
            generate_access_token(github_app_secret.as_ref()).await?;

        Ok(Self {
            access_token: Arc::new(RwLock::new(access_token)),
            installation_access_tokens: Default::default(),
            github_app_secret,
            http_client: reqwest::Client::new(),
        })
    }

    async fn get_access_token(&self) -> Result<String, crate::Error> {
        let token = self.access_token.read().await;
        if Instant::now() < token.expires_at {
            return Ok(token.token.clone());
        }
        drop(token);
        let new_token = generate_access_token(&self.github_app_secret).await?;
        let output = new_token.token.clone();
        *self.access_token.write().await = new_token;
        Ok(output)
    }

    #[instrument(skip_all)]
    async fn fetch_installation_access_token(
        &self,
        installation_id: u64,
    ) -> Result<CachedToken, crate::Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            token: String,
        }

        let access_token = self.get_access_token().await?;
        let expires_at = Instant::now() + Duration::from_mins(30);
        let response = self
            .http_client
            .post(format!(
                "https://api.github.com/app/installations/{}/access_tokens",
                installation_id
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .bytes()
            .await?;

        let response: Response = serde_json::from_slice(&response)
            .inspect_err(|err| {
                error!(message = "Failed to deserialize response",
                    error = ?err,
                    body = ?String::from_utf8_lossy(&response)
                );
            })?;

        Ok(CachedToken::new(&response.token, expires_at))
    }

    #[allow(dead_code)]
    pub async fn get_installtion_access_token(
        &self,
        installation_id: u64,
    ) -> Result<String, crate::Error> {
        if let Some(token) = self
            .installation_access_tokens
            .read()
            .await
            .get(&installation_id)
            && Instant::now() > token.expires_at
        {
            return Ok(token.token.clone());
        };
        let installation_access_token = self
            .fetch_installation_access_token(installation_id)
            .await?;

        let output = installation_access_token.token.clone();
        self.installation_access_tokens
            .write()
            .await
            .insert(installation_id, installation_access_token);

        Ok(output)
    }
}
