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

/// ProviderRequest : Provider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// Flow used for authentication when the associated application is accessed by an un-authenticated user.
    #[serde(
        rename = "authentication_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_flow: Option<Option<uuid::Uuid>>,
    /// Flow used when authorizing this provider.
    #[serde(rename = "authorization_flow")]
    pub authorization_flow: uuid::Uuid,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
}

impl ProviderRequest {
    /// Provider Serializer
    pub fn new(name: String, authorization_flow: uuid::Uuid) -> ProviderRequest {
        ProviderRequest {
            name,
            authentication_flow: None,
            authorization_flow,
            property_mappings: None,
        }
    }
}
