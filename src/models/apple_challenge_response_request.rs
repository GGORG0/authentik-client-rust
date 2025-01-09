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

/// AppleChallengeResponseRequest : Pseudo class for apple response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppleChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
}

impl AppleChallengeResponseRequest {
    /// Pseudo class for apple response
    pub fn new() -> AppleChallengeResponseRequest {
        AppleChallengeResponseRequest { component: None }
    }
}
