/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedPasswordStageRequest : PasswordStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedPasswordStageRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// Selection of backends to test the password against.
    #[serde(rename = "backends", skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<models::BackendsEnum>>,
    /// Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage.
    #[serde(
        rename = "configure_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub configure_flow: Option<Option<uuid::Uuid>>,
    /// How many attempts a user has before the flow is canceled. To lock the user out, use a reputation policy and a user_write stage.
    #[serde(rename = "failed_attempts_before_cancel", skip_serializing_if = "Option::is_none")]
    pub failed_attempts_before_cancel: Option<i32>,
    /// When enabled, provides a 'show password' button with the password input field.
    #[serde(rename = "allow_show_password", skip_serializing_if = "Option::is_none")]
    pub allow_show_password: Option<bool>,
}

impl PatchedPasswordStageRequest {
    /// PasswordStage Serializer
    pub fn new() -> PatchedPasswordStageRequest {
        PatchedPasswordStageRequest {
            name: None,
            flow_set: None,
            backends: None,
            configure_flow: None,
            failed_attempts_before_cancel: None,
            allow_show_password: None,
        }
    }
}
