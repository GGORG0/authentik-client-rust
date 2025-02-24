/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GoogleWorkspaceProviderGroupRequest : GoogleWorkspaceProviderGroup Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GoogleWorkspaceProviderGroupRequest {
    #[serde(rename = "google_id")]
    pub google_id: String,
    #[serde(rename = "group")]
    pub group: uuid::Uuid,
    #[serde(rename = "provider")]
    pub provider: i32,
}

impl GoogleWorkspaceProviderGroupRequest {
    /// GoogleWorkspaceProviderGroup Serializer
    pub fn new(google_id: String, group: uuid::Uuid, provider: i32) -> GoogleWorkspaceProviderGroupRequest {
        GoogleWorkspaceProviderGroupRequest {
            google_id,
            group,
            provider,
        }
    }
}
