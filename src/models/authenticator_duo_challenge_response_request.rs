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

/// AuthenticatorDuoChallengeResponseRequest : Pseudo class for duo response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorDuoChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
}

impl AuthenticatorDuoChallengeResponseRequest {
    /// Pseudo class for duo response
    pub fn new() -> AuthenticatorDuoChallengeResponseRequest {
        AuthenticatorDuoChallengeResponseRequest { component: None }
    }
}
