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

/// PatchedInvitationStageRequest : InvitationStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedInvitationStageRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// If this flag is set, this Stage will jump to the next Stage when no Invitation is given. By default this Stage will cancel the Flow when no invitation is given.
    #[serde(rename = "continue_flow_without_invitation", skip_serializing_if = "Option::is_none")]
    pub continue_flow_without_invitation: Option<bool>,
}

impl PatchedInvitationStageRequest {
    /// InvitationStage Serializer
    pub fn new() -> PatchedInvitationStageRequest {
        PatchedInvitationStageRequest {
            name: None,
            flow_set: None,
            continue_flow_without_invitation: None,
        }
    }
}
