/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedUserPlexSourceConnectionRequest : Plex Source connection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedUserPlexSourceConnectionRequest {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<i32>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<uuid::Uuid>,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "plex_token", skip_serializing_if = "Option::is_none")]
    pub plex_token: Option<String>,
}

impl PatchedUserPlexSourceConnectionRequest {
    /// Plex Source connection Serializer
    pub fn new() -> PatchedUserPlexSourceConnectionRequest {
        PatchedUserPlexSourceConnectionRequest {
            user: None,
            source: None,
            identifier: None,
            plex_token: None,
        }
    }
}
