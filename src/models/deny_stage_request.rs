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

/// DenyStageRequest : DenyStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DenyStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    #[serde(rename = "deny_message", skip_serializing_if = "Option::is_none")]
    pub deny_message: Option<String>,
}

impl DenyStageRequest {
    /// DenyStage Serializer
    pub fn new(name: String) -> DenyStageRequest {
        DenyStageRequest {
            name,
            flow_set: None,
            deny_message: None,
        }
    }
}
