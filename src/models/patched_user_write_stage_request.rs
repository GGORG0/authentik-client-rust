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

/// PatchedUserWriteStageRequest : UserWriteStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedUserWriteStageRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    #[serde(rename = "user_creation_mode", skip_serializing_if = "Option::is_none")]
    pub user_creation_mode: Option<models::UserCreationModeEnum>,
    /// When set, newly created users are inactive and cannot login.
    #[serde(rename = "create_users_as_inactive", skip_serializing_if = "Option::is_none")]
    pub create_users_as_inactive: Option<bool>,
    /// Optionally add newly created users to this group.
    #[serde(
        rename = "create_users_group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_users_group: Option<Option<uuid::Uuid>>,
    #[serde(rename = "user_type", skip_serializing_if = "Option::is_none")]
    pub user_type: Option<models::UserTypeEnum>,
    #[serde(rename = "user_path_template", skip_serializing_if = "Option::is_none")]
    pub user_path_template: Option<String>,
}

impl PatchedUserWriteStageRequest {
    /// UserWriteStage Serializer
    pub fn new() -> PatchedUserWriteStageRequest {
        PatchedUserWriteStageRequest {
            name: None,
            flow_set: None,
            user_creation_mode: None,
            create_users_as_inactive: None,
            create_users_group: None,
            user_type: None,
            user_path_template: None,
        }
    }
}
