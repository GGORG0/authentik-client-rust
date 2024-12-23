/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserDeleteStageRequest : UserDeleteStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDeleteStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
}

impl UserDeleteStageRequest {
    /// UserDeleteStage Serializer
    pub fn new(name: String) -> UserDeleteStageRequest {
        UserDeleteStageRequest { name, flow_set: None }
    }
}
