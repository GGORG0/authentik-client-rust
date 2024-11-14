/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ScimProviderGroup : SCIMProviderGroup Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScimProviderGroup {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "scim_id")]
    pub scim_id: String,
    #[serde(rename = "group")]
    pub group: uuid::Uuid,
    #[serde(rename = "group_obj")]
    pub group_obj: models::UserGroup,
    #[serde(rename = "provider")]
    pub provider: i32,
}

impl ScimProviderGroup {
    /// SCIMProviderGroup Serializer
    pub fn new(
        id: uuid::Uuid,
        scim_id: String,
        group: uuid::Uuid,
        group_obj: models::UserGroup,
        provider: i32,
    ) -> ScimProviderGroup {
        ScimProviderGroup {
            id,
            scim_id,
            group,
            group_obj,
            provider,
        }
    }
}
