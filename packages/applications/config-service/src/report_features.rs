use std::collections::{BTreeMap, BTreeSet};

use axum::{
    Json,
    extract::{FromRequestParts, State},
    http::StatusCode,
};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use tracing::{error, info, instrument};

use crate::{
    database::{
        get_organization::GetOrganizationByOrganizationIdQueryBuilder,
        get_repository::GetRepositoryByOrganizationIdAndRepositoryIdBuilder,
    },
    error::ServiceError,
    state::ServiceState,
};

const AUDIENCE: &str = "https://api.arithmico.com/github/features";

#[derive(Debug, Deserialize)]
pub struct GithubIdTokenClaims {
    repository_id: String,
    r#ref: String,
    ref_type: String,
    repository_owner_id: String,
}

impl FromRequestParts<ServiceState> for GithubIdTokenClaims {
    type Rejection = StatusCode;

    #[instrument(skip_all)]
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &ServiceState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or_else(|| {
                error!("Missing authorization header");
                StatusCode::UNAUTHORIZED
            })?
            .to_str()
            .map_err(|err| {
                error!(message = "Invalid authorization header", error = ?err);
                StatusCode::UNAUTHORIZED
            })?;

        let token = token.strip_prefix("bearer ").unwrap_or(token);

        let claims: GithubIdTokenClaims = state
            .github_oidc_client
            .decode(AUDIENCE, token)
            .await
            .map_err(|err| {
                error!(message = "Invalid token", error = ?err);
                StatusCode::UNAUTHORIZED
            })?;

        Ok(claims)
    }
}

// TODO: move dto types into shared library
type RequestBody = BTreeMap<String, BTreeSet<String>>;

pub async fn handler(
    State(state): State<ServiceState>,
    claims: GithubIdTokenClaims,
    Json(body): Json<RequestBody>,
) -> Result<(), ServiceError> {
    if claims.ref_type != "tag" {
        error!(message = "Ref is not a tag", ref_type = claims.ref_type);
        return Err(ServiceError::BadRequest);
    }
    let Some(tag) = claims.r#ref.strip_prefix("refs/tags/v") else {
        error!(message = "Invalid tag name", r#ref = claims.r#ref);
        return Err(ServiceError::BadRequest);
    };
    let organization_id: u64 =
        claims.repository_owner_id.parse().map_err(|err| {
            error!(
                message = "Failed to parse organization_id",
                organization_id = claims.repository_owner_id,
                error = ?err
            );
            ServiceError::BadRequest
        })?;

    let repository_id: u64 = claims.repository_id.parse().map_err(|err| {
        error!(
            message = "Failed to parse organization_id",
            repository_id= claims.repository_id,
            error = ?err
        );
        ServiceError::BadRequest
    })?;

    state
        .database
        .execute(
            GetOrganizationByOrganizationIdQueryBuilder::default()
                .organization_id(organization_id)
                .build()?,
        )
        .await
        .map_err(|err| {
            error!(
                message = "Failed to find organization",
                organization_id = organization_id,
                error = ?err
            );
            ServiceError::NotFound
        })?;

    state
        .database
        .execute(
            GetRepositoryByOrganizationIdAndRepositoryIdBuilder::default()
                .organization_id(organization_id)
                .repository_id(repository_id)
                .build()?,
        )
        .await
        .map_err(|err| {
            error!(message = "Failed to find repository", error = ?err);
            ServiceError::NotFound
        })?;

    info!(message = "Received tag features", tag = tag);

    Ok(())
}
