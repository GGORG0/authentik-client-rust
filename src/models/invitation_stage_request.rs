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

/// InvitationStageRequest : InvitationStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvitationStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// If this flag is set, this Stage will jump to the next Stage when no Invitation is given. By default this Stage will cancel the Flow when no invitation is given.
    #[serde(rename = "continue_flow_without_invitation", skip_serializing_if = "Option::is_none")]
    pub continue_flow_without_invitation: Option<bool>,
}

impl InvitationStageRequest {
    /// InvitationStage Serializer
    pub fn new(name: String) -> InvitationStageRequest {
        InvitationStageRequest {
            name,
            flow_set: None,
            continue_flow_without_invitation: None,
        }
    }
}
