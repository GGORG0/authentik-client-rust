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

/// DummyChallengeResponseRequest : Dummy challenge response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DummyChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
}

impl DummyChallengeResponseRequest {
    /// Dummy challenge response
    pub fn new() -> DummyChallengeResponseRequest {
        DummyChallengeResponseRequest { component: None }
    }
}
