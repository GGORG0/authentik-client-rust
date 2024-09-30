/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.8.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedConnectionTokenRequest : ConnectionToken Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedConnectionTokenRequest {
    #[serde(rename = "pk", skip_serializing_if = "Option::is_none")]
    pub pk: Option<uuid::Uuid>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<i32>,
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<uuid::Uuid>,
}

impl PatchedConnectionTokenRequest {
    /// ConnectionToken Serializer
    pub fn new() -> PatchedConnectionTokenRequest {
        PatchedConnectionTokenRequest {
            pk: None,
            provider: None,
            endpoint: None,
        }
    }
}
