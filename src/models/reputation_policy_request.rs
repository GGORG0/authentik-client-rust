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

/// ReputationPolicyRequest : Reputation Policy Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReputationPolicyRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged.
    #[serde(rename = "execution_logging", skip_serializing_if = "Option::is_none")]
    pub execution_logging: Option<bool>,
    #[serde(rename = "check_ip", skip_serializing_if = "Option::is_none")]
    pub check_ip: Option<bool>,
    #[serde(rename = "check_username", skip_serializing_if = "Option::is_none")]
    pub check_username: Option<bool>,
    #[serde(rename = "threshold", skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i32>,
}

impl ReputationPolicyRequest {
    /// Reputation Policy Serializer
    pub fn new(name: String) -> ReputationPolicyRequest {
        ReputationPolicyRequest {
            name,
            execution_logging: None,
            check_ip: None,
            check_username: None,
            threshold: None,
        }
    }
}
