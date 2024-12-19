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

/// IdentificationChallengeResponseRequest : Identification challenge
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentificationChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "uid_field")]
    pub uid_field: String,
    #[serde(
        rename = "password",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub password: Option<Option<String>>,
    #[serde(
        rename = "captcha_token",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub captcha_token: Option<Option<String>>,
}

impl IdentificationChallengeResponseRequest {
    /// Identification challenge
    pub fn new(uid_field: String) -> IdentificationChallengeResponseRequest {
        IdentificationChallengeResponseRequest {
            component: None,
            uid_field,
            password: None,
            captcha_token: None,
        }
    }
}
