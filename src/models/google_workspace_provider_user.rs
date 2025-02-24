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

/// GoogleWorkspaceProviderUser : GoogleWorkspaceProviderUser Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GoogleWorkspaceProviderUser {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "google_id")]
    pub google_id: String,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "user_obj")]
    pub user_obj: models::GroupMember,
    #[serde(rename = "provider")]
    pub provider: i32,
    #[serde(rename = "attributes", deserialize_with = "Option::deserialize")]
    pub attributes: Option<serde_json::Value>,
}

impl GoogleWorkspaceProviderUser {
    /// GoogleWorkspaceProviderUser Serializer
    pub fn new(
        id: uuid::Uuid,
        google_id: String,
        user: i32,
        user_obj: models::GroupMember,
        provider: i32,
        attributes: Option<serde_json::Value>,
    ) -> GoogleWorkspaceProviderUser {
        GoogleWorkspaceProviderUser {
            id,
            google_id,
            user,
            user_obj,
            provider,
            attributes,
        }
    }
}
