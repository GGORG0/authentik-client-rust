/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.8.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// MicrosoftEntraProviderUser : MicrosoftEntraProviderUser Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftEntraProviderUser {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "microsoft_id")]
    pub microsoft_id: String,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "user_obj")]
    pub user_obj: models::GroupMember,
    #[serde(rename = "provider")]
    pub provider: i32,
    #[serde(rename = "attributes", deserialize_with = "Option::deserialize")]
    pub attributes: Option<serde_json::Value>,
}

impl MicrosoftEntraProviderUser {
    /// MicrosoftEntraProviderUser Serializer
    pub fn new(
        id: uuid::Uuid,
        microsoft_id: String,
        user: i32,
        user_obj: models::GroupMember,
        provider: i32,
        attributes: Option<serde_json::Value>,
    ) -> MicrosoftEntraProviderUser {
        MicrosoftEntraProviderUser {
            id,
            microsoft_id,
            user,
            user_obj,
            provider,
            attributes,
        }
    }
}
