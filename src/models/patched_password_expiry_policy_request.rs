/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedPasswordExpiryPolicyRequest : Password Expiry Policy Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedPasswordExpiryPolicyRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged.
    #[serde(rename = "execution_logging", skip_serializing_if = "Option::is_none")]
    pub execution_logging: Option<bool>,
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(rename = "deny_only", skip_serializing_if = "Option::is_none")]
    pub deny_only: Option<bool>,
}

impl PatchedPasswordExpiryPolicyRequest {
    /// Password Expiry Policy Serializer
    pub fn new() -> PatchedPasswordExpiryPolicyRequest {
        PatchedPasswordExpiryPolicyRequest {
            name: None,
            execution_logging: None,
            days: None,
            deny_only: None,
        }
    }
}
