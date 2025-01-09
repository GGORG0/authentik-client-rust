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

/// PatchedUserLogoutStageRequest : UserLogoutStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedUserLogoutStageRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
}

impl PatchedUserLogoutStageRequest {
    /// UserLogoutStage Serializer
    pub fn new() -> PatchedUserLogoutStageRequest {
        PatchedUserLogoutStageRequest {
            name: None,
            flow_set: None,
        }
    }
}
