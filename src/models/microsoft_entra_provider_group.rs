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

/// MicrosoftEntraProviderGroup : MicrosoftEntraProviderGroup Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftEntraProviderGroup {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "microsoft_id")]
    pub microsoft_id: String,
    #[serde(rename = "group")]
    pub group: uuid::Uuid,
    #[serde(rename = "group_obj")]
    pub group_obj: models::UserGroup,
    #[serde(rename = "provider")]
    pub provider: i32,
    #[serde(rename = "attributes", deserialize_with = "Option::deserialize")]
    pub attributes: Option<serde_json::Value>,
}

impl MicrosoftEntraProviderGroup {
    /// MicrosoftEntraProviderGroup Serializer
    pub fn new(
        id: uuid::Uuid,
        microsoft_id: String,
        group: uuid::Uuid,
        group_obj: models::UserGroup,
        provider: i32,
        attributes: Option<serde_json::Value>,
    ) -> MicrosoftEntraProviderGroup {
        MicrosoftEntraProviderGroup {
            id,
            microsoft_id,
            group,
            group_obj,
            provider,
            attributes,
        }
    }
}
