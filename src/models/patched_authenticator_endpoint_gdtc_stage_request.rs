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

/// PatchedAuthenticatorEndpointGdtcStageRequest : AuthenticatorEndpointGDTCStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedAuthenticatorEndpointGdtcStageRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    #[serde(
        rename = "credentials",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub credentials: Option<Option<serde_json::Value>>,
}

impl PatchedAuthenticatorEndpointGdtcStageRequest {
    /// AuthenticatorEndpointGDTCStage Serializer
    pub fn new() -> PatchedAuthenticatorEndpointGdtcStageRequest {
        PatchedAuthenticatorEndpointGdtcStageRequest {
            name: None,
            flow_set: None,
            configure_flow: None,
            friendly_name: None,
            credentials: None,
        }
    }
}
