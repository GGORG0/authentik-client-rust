/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedUserSourceConnectionRequest : User source connection
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedUserSourceConnectionRequest {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<i32>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<uuid::Uuid>,
}

impl PatchedUserSourceConnectionRequest {
    /// User source connection
    pub fn new() -> PatchedUserSourceConnectionRequest {
        PatchedUserSourceConnectionRequest {
            user: None,
            source: None,
        }
    }
}
