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

/// PatchedUserLoginStageRequest : UserLoginStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedUserLoginStageRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// Determines how long a session lasts. Default of 0 means that the sessions lasts until the browser is closed. (Format: hours=-1;minutes=-2;seconds=-3)
    #[serde(rename = "session_duration", skip_serializing_if = "Option::is_none")]
    pub session_duration: Option<String>,
    /// Terminate all other sessions of the user logging in.
    #[serde(rename = "terminate_other_sessions", skip_serializing_if = "Option::is_none")]
    pub terminate_other_sessions: Option<bool>,
    /// Offset the session will be extended by when the user picks the remember me option. Default of 0 means that the remember me option will not be shown. (Format: hours=-1;minutes=-2;seconds=-3)
    #[serde(rename = "remember_me_offset", skip_serializing_if = "Option::is_none")]
    pub remember_me_offset: Option<String>,
    /// Bind sessions created by this stage to the configured network
    #[serde(rename = "network_binding", skip_serializing_if = "Option::is_none")]
    pub network_binding: Option<models::NetworkBindingEnum>,
    /// Bind sessions created by this stage to the configured GeoIP location
    #[serde(rename = "geoip_binding", skip_serializing_if = "Option::is_none")]
    pub geoip_binding: Option<models::GeoipBindingEnum>,
}

impl PatchedUserLoginStageRequest {
    /// UserLoginStage Serializer
    pub fn new() -> PatchedUserLoginStageRequest {
        PatchedUserLoginStageRequest {
            name: None,
            flow_set: None,
            session_duration: None,
            terminate_other_sessions: None,
            remember_me_offset: None,
            network_binding: None,
            geoip_binding: None,
        }
    }
}
