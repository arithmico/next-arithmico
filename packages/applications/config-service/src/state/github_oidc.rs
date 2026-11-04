use std::sync::Arc;

use anyhow::anyhow;
use jsonwebtoken::{
    DecodingKey, TokenData, Validation, decode, decode_header,
    jwk::{Jwk, JwkSet},
};
use serde::Deserialize;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct GithubOidcClient {
    jwks: Arc<RwLock<JwkSet>>,
    jwks_uri: String,
    issuer: String,
}

#[derive(Debug, Deserialize)]
struct OidcConfiguration {
    pub issuer: String,
    pub jwks_uri: String,
}

impl GithubOidcClient {
    pub async fn init(oidc_configuration_uri: &str) -> anyhow::Result<Self> {
        let config: OidcConfiguration =
            reqwest::get(oidc_configuration_uri).await?.json().await?;

        let instance = Self {
            jwks: Default::default(),
            jwks_uri: config.jwks_uri,
            issuer: config.issuer,
        };

        Ok(instance)
    }

    async fn refetch_jwks(&self) -> anyhow::Result<()> {
        let jwks: JwkSet = reqwest::get(&self.jwks_uri).await?.json().await?;
        *self.jwks.write().await = jwks;
        Ok(())
    }

    async fn get_jwk(&self, kid: &str) -> anyhow::Result<Jwk> {
        if let Some(jwk) = self.jwks.read().await.find(kid) {
            return Ok(jwk.clone());
        }
        // retry after refetch
        self.refetch_jwks().await?;
        if let Some(jwk) = self.jwks.read().await.find(kid) {
            return Ok(jwk.clone());
        }
        Err(anyhow!("Failed to lookup jwk for kid: {}", kid))
    }

    async fn get_decoding_key(&self, kid: &str) -> anyhow::Result<DecodingKey> {
        let jwk = self.get_jwk(kid).await?;
        Ok(DecodingKey::from_jwk(&jwk)?)
    }

    pub async fn decode<T: for<'de> Deserialize<'de>>(
        &self,
        audience: &str,
        token: &str,
    ) -> anyhow::Result<T> {
        let header = decode_header(token)?;
        let Some(kid) = header.kid.as_ref() else {
            return Err(anyhow!("Header does not contain a kid"));
        };
        let decoding_key = self.get_decoding_key(kid).await?;
        let mut validation = Validation::new(header.alg);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[audience]);
        let data: TokenData<T> = decode(token, &decoding_key, &validation)?;
        Ok(data.claims)
    }
}
