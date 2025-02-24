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

/// MicrosoftEntraProviderGroupRequest : MicrosoftEntraProviderGroup Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftEntraProviderGroupRequest {
    #[serde(rename = "microsoft_id")]
    pub microsoft_id: String,
    #[serde(rename = "group")]
    pub group: uuid::Uuid,
    #[serde(rename = "provider")]
    pub provider: i32,
}

impl MicrosoftEntraProviderGroupRequest {
    /// MicrosoftEntraProviderGroup Serializer
    pub fn new(microsoft_id: String, group: uuid::Uuid, provider: i32) -> MicrosoftEntraProviderGroupRequest {
        MicrosoftEntraProviderGroupRequest {
            microsoft_id,
            group,
            provider,
        }
    }
}
