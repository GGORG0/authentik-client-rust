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

/// PolicyBinding : PolicyBinding Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyBinding {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
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
    #[serde(rename = "policy_obj")]
    pub policy_obj: models::Policy,
    #[serde(rename = "group_obj")]
    pub group_obj: models::Group,
    #[serde(rename = "user_obj")]
    pub user_obj: models::User,
    #[serde(rename = "target")]
    pub target: uuid::Uuid,
    /// Negates the outcome of the policy. Messages are unaffected.
    #[serde(rename = "negate", skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "order")]
    pub order: i32,
    /// Timeout after which Policy execution is terminated.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    /// Result if the Policy execution fails.
    #[serde(rename = "failure_result", skip_serializing_if = "Option::is_none")]
    pub failure_result: Option<bool>,
}

impl PolicyBinding {
    /// PolicyBinding Serializer
    pub fn new(
        pk: uuid::Uuid,
        policy_obj: models::Policy,
        group_obj: models::Group,
        user_obj: models::User,
        target: uuid::Uuid,
        order: i32,
    ) -> PolicyBinding {
        PolicyBinding {
            pk,
            policy: None,
            group: None,
            user: None,
            policy_obj,
            group_obj,
            user_obj,
            target,
            negate: None,
            enabled: None,
            order,
            timeout: None,
            failure_result: None,
        }
    }
}
