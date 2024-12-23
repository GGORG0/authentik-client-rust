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

/// ScimProviderUser : SCIMProviderUser Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScimProviderUser {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "scim_id")]
    pub scim_id: String,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "user_obj")]
    pub user_obj: models::GroupMember,
    #[serde(rename = "provider")]
    pub provider: i32,
}

impl ScimProviderUser {
    /// SCIMProviderUser Serializer
    pub fn new(
        id: uuid::Uuid,
        scim_id: String,
        user: i32,
        user_obj: models::GroupMember,
        provider: i32,
    ) -> ScimProviderUser {
        ScimProviderUser {
            id,
            scim_id,
            user,
            user_obj,
            provider,
        }
    }
}
