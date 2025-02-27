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

/// OAuthDeviceCodeChallengeResponseRequest : Response that includes the user-entered device code
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthDeviceCodeChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "code")]
    pub code: String,
}

impl OAuthDeviceCodeChallengeResponseRequest {
    /// Response that includes the user-entered device code
    pub fn new(code: String) -> OAuthDeviceCodeChallengeResponseRequest {
        OAuthDeviceCodeChallengeResponseRequest { component: None, code }
    }
}
