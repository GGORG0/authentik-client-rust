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

/// PatchedPolicyBindingRequest : PolicyBinding Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedPolicyBindingRequest {
    #[serde(
        rename = "policy",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub policy: Option<Option<uuid::Uuid>>,
    #[serde(
        rename = "group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub group: Option<Option<uuid::Uuid>>,
    #[serde(
        rename = "user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<Option<i32>>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<uuid::Uuid>,
    /// Negates the outcome of the policy. Messages are unaffected.
    #[serde(rename = "negate", skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// Timeout after which Policy execution is terminated.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    /// Result if the Policy execution fails.
    #[serde(rename = "failure_result", skip_serializing_if = "Option::is_none")]
    pub failure_result: Option<bool>,
}

impl PatchedPolicyBindingRequest {
    /// PolicyBinding Serializer
    pub fn new() -> PatchedPolicyBindingRequest {
        PatchedPolicyBindingRequest {
            policy: None,
            group: None,
            user: None,
            target: None,
            negate: None,
            enabled: None,
            order: None,
            timeout: None,
            failure_result: None,
        }
    }
}
