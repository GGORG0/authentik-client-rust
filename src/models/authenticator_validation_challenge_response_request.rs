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

/// AuthenticatorValidationChallengeResponseRequest : Challenge used for Code-based and WebAuthn authenticators
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorValidationChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "selected_challenge", skip_serializing_if = "Option::is_none")]
    pub selected_challenge: Option<models::DeviceChallengeRequest>,
    #[serde(rename = "selected_stage", skip_serializing_if = "Option::is_none")]
    pub selected_stage: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "webauthn", skip_serializing_if = "Option::is_none")]
    pub webauthn: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "duo", skip_serializing_if = "Option::is_none")]
    pub duo: Option<i32>,
}

impl AuthenticatorValidationChallengeResponseRequest {
    /// Challenge used for Code-based and WebAuthn authenticators
    pub fn new() -> AuthenticatorValidationChallengeResponseRequest {
        AuthenticatorValidationChallengeResponseRequest {
            component: None,
            selected_challenge: None,
            selected_stage: None,
            code: None,
            webauthn: None,
            duo: None,
        }
    }
}
