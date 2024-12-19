/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PlexAuthenticationChallengeResponseRequest : Pseudo class for plex response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlexAuthenticationChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
}

impl PlexAuthenticationChallengeResponseRequest {
    /// Pseudo class for plex response
    pub fn new() -> PlexAuthenticationChallengeResponseRequest {
        PlexAuthenticationChallengeResponseRequest { component: None }
    }
}
