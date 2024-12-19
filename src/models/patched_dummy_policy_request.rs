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

/// PatchedDummyPolicyRequest : Dummy Policy Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedDummyPolicyRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged.
    #[serde(rename = "execution_logging", skip_serializing_if = "Option::is_none")]
    pub execution_logging: Option<bool>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
    #[serde(rename = "wait_min", skip_serializing_if = "Option::is_none")]
    pub wait_min: Option<i32>,
    #[serde(rename = "wait_max", skip_serializing_if = "Option::is_none")]
    pub wait_max: Option<i32>,
}

impl PatchedDummyPolicyRequest {
    /// Dummy Policy Serializer
    pub fn new() -> PatchedDummyPolicyRequest {
        PatchedDummyPolicyRequest {
            name: None,
            execution_logging: None,
            result: None,
            wait_min: None,
            wait_max: None,
        }
    }
}
