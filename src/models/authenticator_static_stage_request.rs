/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AuthenticatorStaticStageRequest : AuthenticatorStaticStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorStaticStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage.
    #[serde(
        rename = "configure_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub configure_flow: Option<Option<uuid::Uuid>>,
    #[serde(
        rename = "friendly_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<Option<String>>,
    #[serde(rename = "token_count", skip_serializing_if = "Option::is_none")]
    pub token_count: Option<u32>,
    #[serde(rename = "token_length", skip_serializing_if = "Option::is_none")]
    pub token_length: Option<u32>,
}

impl AuthenticatorStaticStageRequest {
    /// AuthenticatorStaticStage Serializer
    pub fn new(name: String) -> AuthenticatorStaticStageRequest {
        AuthenticatorStaticStageRequest {
            name,
            flow_set: None,
            configure_flow: None,
            friendly_name: None,
            token_count: None,
            token_length: None,
        }
    }
}
